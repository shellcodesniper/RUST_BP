#![allow(dead_code)]

use lazy_static::lazy_static;

use std::sync::Arc;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};
use tokio::sync::RwLock;
use std::ops::Deref;

enum WrapIt<'a, T>{
    Read(RwLockReadGuard<'a, T>),
    Write(RwLockWriteGuard<'a, T>)
}

impl<'a, T> Deref for WrapIt<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            WrapIt::Read(r_g) => r_g.deref(),
            WrapIt::Write(w_g) => w_g.deref()
        }
    }
}

lazy_static! {
  pub static ref GLOBAL_VAR: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
}

impl GLOBAL_VAR{
  pub async fn get(&self) -> Vec<String> {
    let x = self.read().await;
    (*x).clone()
  }

  pub async fn pop(&self) -> Option<String> {
    let mut next = (*self).write().await;
    next.pop()
  }

  pub async fn flush(&self) {
    let mut next = (*self).write().await;
    next.clear();
  }

  pub async fn push(&self, code: String) {
    let mut next = (*self).write().await;
    next.push(code)
  }
}
