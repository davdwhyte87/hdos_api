package com.hdos.api.repositories

import com.hdos.api.model.TestData
import org.springframework.data.mongodb.repository.MongoRepository

interface TestDataRepository : MongoRepository<String, TestData>{
}