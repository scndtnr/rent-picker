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
}
