package com.hdos.api.model

import org.bson.types.ObjectId

data class TestRecord(
    val id:ObjectId = ObjectId.get(),
    val userID:String,
    val testDatas: String,
    val createdAt: String,
    val updatedAt:String,
    val nurseID:String
    )
