package com.hdos.api.model.req

data class ConfirmUserReq(
    var id:String,
    var authCode:String
)