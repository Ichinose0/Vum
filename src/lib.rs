pub enum Request {
    UpdateWindowState,
    UpdateWindowUI,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id(&'static str);

impl Id {
    pub fn new(id: &str) -> Self {
        Self(id)
    }
}

pub trait Widget {
    fn id(&self) -> Id;
}

pub trait Component {
    
}

pub struct ViewModel<T> 
where
    T: Component
{

}

pub trait Window<T> 
where
    T: Component
{
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn title(&self) -> &str;
    fn dispatch(&mut self) -> Request;
    fn ui(&self) -> ViewModel<T>;
}

pub struct Application {

}