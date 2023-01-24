package com.hdos.api.services

import com.hdos.api.model.TestRecord
import com.hdos.api.repositories.TestRecordRepository
import org.springframework.data.mongodb.core.MongoTemplate
import org.springframework.data.mongodb.repository.MongoRepository
import org.springframework.stereotype.Service


@Service
class TestRecordService(
    private val _testRecordRepository:TestRecordRepository,
    private  val mongoTemplate: MongoTemplate
) {
    var testRecordRepository:TestRecordRepository = _testRecordRepository

}