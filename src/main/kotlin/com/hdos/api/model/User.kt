package com.hdos.api.model

import org.bson.types.ObjectId
import org.springframework.data.annotation.Id

data class User(
    @Id
    val id:ObjectId = ObjectId.get(),
    val name:String,
    val email:String,
    val authCode:String,
    var createdAt:String?,
    var isConfirmed:Boolean = false,
    val role: Role
)
