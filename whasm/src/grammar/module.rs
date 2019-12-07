use super::*;
use super::section::*;

#[derive(Debug, PartialEq, Grammar)]
pub struct Module {
    #[matching(magic if magic == ['\0', 'a', 's', 'm'])]
    pub magic: [Byte; 4],

    #[matching(ver if ver == [1, 0, 0, 0])]
    pub version: [Byte; 4],

    pub sections: List<Section>,
}