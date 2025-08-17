// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::BoxError;
use bytes::Bytes;
use http_body_util::BodyExt;

/*
BoxBody 是 http_body_util 库中的一个类型，用于将一个 Body 包装成一个 Box 类型。
它实现了 http_body::Body trait，可以用于处理 HTTP 请求和响应的 body 部分。
*/

pub type BoxBody = http_body_util::combinators::UnsyncBoxBody<Bytes, BoxError>;

pub fn boxed<B>(body: B) -> BoxBody
where
    B: http_body::Body<Data = Bytes> + Send + 'static,
    B::Error: Into<BoxError>,
{
    try_downcast(body).unwrap_or_else(|body| body.map_err(Into::into).boxed_unsync())
}

pub(crate) fn try_downcast<T, K>(k: K) -> Result<T, K>
where
    T: 'static,
    K: Send + 'static,
{
    //执行向下转换，将K转换为T
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}
