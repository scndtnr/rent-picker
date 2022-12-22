use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use scraper::{ElementRef, Html};

pub trait HtmlParser {
    fn innter_text_of_element(&self, html: &str, css_selector: &str, sep: &str) -> String {
        let html = self.parse_html(html);
        let element = self.find_element(&html, css_selector).unwrap();
        self.inner_text(&element, sep)
    }

    fn find_inner_text(&self, html: &Html, css_selector: &str, sep: &str) -> String {
        let element = self.find_element(html, css_selector).unwrap();
        self.inner_text(&element, sep)
    }

    fn find_inner_text_by_element<'a>(
        &self,
        element: &'a ElementRef,
        css_selector: &str,
        sep: &str,
    ) -> String {
        let element = self
            .find_element_by_element(element, css_selector)
            .with_context(|| format!("Fail to find element. selector: {}", css_selector))
            .unwrap();
        self.inner_text(&element, sep)
    }

    fn find_table_and_parse<'a>(
        &self,
        html: &'a Html,
        css_selector: &str,
    ) -> HashMap<String, String> {
        let table_element = self
            .find_element(html, css_selector)
            .with_context(|| format!("Fail to find element. selector: {}", css_selector))
            .unwrap();
        self.parse_table(&table_element)
    }

    /// 返り値のHtmlはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    fn parse_html(&self, html: &str) -> Html {
        Html::parse_document(html)
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    fn find_elements<'a, 'b>(&self, html: &'a Html, css_selector: &'b str) -> Vec<ElementRef<'a>> {
        let selector = scraper::Selector::parse(css_selector).unwrap();
        html.select(&selector).collect::<Vec<ElementRef>>()
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    fn find_element<'a, 'b>(
        &self,
        html: &'a Html,
        css_selector: &'b str,
    ) -> Result<ElementRef<'a>> {
        let elems = self.find_elements(html, css_selector);
        if elems.is_empty() {
            bail!("Element is not found. selector: {}", css_selector);
        } else if elems.len() == 1 {
            Ok(elems[0])
        } else {
            bail!("Multiple elements are found. selector: {}", css_selector);
        }
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    fn find_elements_by_element<'a, 'b>(
        &self,
        element: &'a ElementRef,
        css_selector: &'b str,
    ) -> Vec<ElementRef<'a>> {
        let selector = scraper::Selector::parse(css_selector).unwrap();
        element.select(&selector).collect::<Vec<ElementRef>>()
    }

    /// 返り値のElementRefはthreads safeではないため、
    /// async内で利用する場合は{}で囲ってスコープを変える必要がある
    fn find_element_by_element<'a, 'b>(
        &self,
        element: &'a ElementRef,
        css_selector: &'b str,
    ) -> Result<ElementRef<'a>> {
        let elems = self.find_elements_by_element(element, css_selector);
        if elems.is_empty() {
            bail!("Element is not found. selector: {}", css_selector);
        } else if elems.len() == 1 {
            Ok(elems[0])
        } else {
            bail!("Multiple elements are found. selector: {}", css_selector);
        }
    }

    fn inner_text(&self, element: &ElementRef, sep: &str) -> String {
        element
            .text()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join(sep)
    }

    fn parse_table(&self, table: &ElementRef) -> HashMap<String, String> {
        let header = self
            .find_elements_by_element(table, "th")
            .into_iter()
            .map(|header| self.inner_text(&header, ""));
        let data = self
            .find_elements_by_element(table, "td")
            .into_iter()
            .map(|data| self.inner_text(&data, "\n"));

        header
            .zip(data)
            .fold(HashMap::new(), |mut acc, (header, data)| {
                acc.insert(header, data);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCrawler;
    impl HtmlParser for TestCrawler {}

    #[test]
    fn htmlのテーブルをhash_mapに直す() {
        // 事前準備
        let crawler = TestCrawler;
        let html = r#"
            <table class="data_table">
                <tbody>
                    <tr>
                        <th>所在地</th>
                        <td>埼玉県</td>
                    </tr>
                    <tr>
                        <th class="data_01" scope="cols">間取り詳細</th>
                        <td>-</td>
                        <th class="data_02" scope="cols">構造</th>
                        <td>鉄骨</td>
                    </tr>
                    <tr>
                        <th class="data_01" scope="cols">階建</th>
                        <td>3階/3階建</td>
                        <th class="data_02" scope="cols">築年月</th>
                        <td>1996年5月</td>
                    </tr>
                    <tr>
                        <th class="property_view_table-title">駅徒歩</th>
                            <td colspan="3" class="property_view_table-body">
                                <div class="property_view_table-read">A駅 歩18分</div>
                                <div class="property_view_table-read">B駅 歩20分</div>
                                <div class="property_view_table-read">C駅 歩18分</div>
                            </td>
                    </tr>
                    <tr>
                        <th class="data_02" scope="cols">取り扱い店舗<br>物件コード</th>
                        <td>test_code_a</td>
                    </tr>
                    <tr>
                        <th class="data_01" scope="cols">SUUMO<br>物件コード</th>
                        <td>test_code_b</td>
                    </tr>
                </tbody>
            </table>
        "#;
        let html = Html::parse_fragment(html);
        let elem = crawler.find_element(&html, "table").unwrap();
        let parsed_table = crawler.parse_table(&elem);

        // 値チェック
        // dbg!(&parsed_table);
        assert_eq!(
            parsed_table.get("所在地").map(|s| s.as_str()),
            Some("埼玉県")
        );
        assert_eq!(
            parsed_table.get("間取り詳細").map(|s| s.as_str()),
            Some("-")
        );
        assert_eq!(parsed_table.get("構造").map(|s| s.as_str()), Some("鉄骨"));
        assert_eq!(
            parsed_table.get("階建").map(|s| s.as_str()),
            Some("3階/3階建")
        );
        assert_eq!(
            parsed_table.get("築年月").map(|s| s.as_str()),
            Some("1996年5月")
        );
        assert_eq!(
            parsed_table.get("駅徒歩").map(|s| s.as_str()),
            Some("A駅 歩18分\nB駅 歩20分\nC駅 歩18分")
        );
        assert_eq!(
            parsed_table
                .get("取り扱い店舗物件コード")
                .map(|s| s.as_str()),
            Some("test_code_a")
        );
        assert_eq!(
            parsed_table.get("SUUMO物件コード").map(|s| s.as_str()),
            Some("test_code_b")
        );
        assert_eq!(parsed_table.get("実在しないキー"), None);
    }
}
