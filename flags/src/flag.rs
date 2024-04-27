/// 给所有 flag 定义的 trait
trait Flag {
    /// 返回 flag 的短名称
    fn name_short(&self) -> Option<u8>;

    /// 返回 flag 的长名称
    fn name_long(&self) -> &'static str;

    /// 返回 flag 的描述信息
    fn desc(&self) -> &'static str;

    /// 将给定的值，更新到 args 中
    fn update(&self, value: String, args: &mut Args);
}
