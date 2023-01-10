package com.hdos.api.model

import org.bson.types.ObjectId

data class TestData(
    val id:ObjectId = ObjectId.get(),
    val name:String,
    val result:String,
    val userID: String,
    val testRecordID:String,
    val createdAt:String,
    val updatedAt:String?,
    val NurseID: String
)
