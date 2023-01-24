package com.hdos.api.services

import com.hdos.api.repositories.TestDataRepository
import com.hdos.api.repositories.UserRepository
import org.springframework.data.mongodb.core.MongoTemplate
import org.springframework.stereotype.Service


@Service
class TestDataService(private val _testDataRepository: TestDataRepository,
                      private val mongoTemplate: MongoTemplate) {
    var testDataRepository:TestDataRepository = _testDataRepository
}