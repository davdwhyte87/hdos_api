package com.hdos.api.repositories

import com.hdos.api.model.TestRecord
import org.springframework.data.mongodb.repository.MongoRepository

interface TestRecordRepository: MongoRepository<TestRecord, String> {
}