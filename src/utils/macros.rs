macro_rules! impl_send_sync {
  ($id:ident) => {
    unsafe impl<'ctx> Send for $id<'ctx> {}

    unsafe impl<'ctx> Sync for $id<'ctx> {}
  }
}