#[derive(Debug)]
pub enum SpecialMode {
    HelpShort,
    HelpLong,
}

/// 参数集
#[derive(Debug, Default)]
pub struct Args {
    /// 特殊参数
    pub special: Option<SpecialMode>,
    /// 正则表达式
    pub patterns: String,
    /// 搜索位置
    pub positional: String,
}
