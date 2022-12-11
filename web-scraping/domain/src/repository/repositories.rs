use super::SuumoRepository;

pub trait Repositories {
    type SuumoRepo: SuumoRepository;

    fn suumo_repository(&self) -> &Self::SuumoRepo;
}
