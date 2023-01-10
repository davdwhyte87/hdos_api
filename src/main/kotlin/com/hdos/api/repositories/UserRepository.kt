package com.hdos.api.repositories

import com.hdos.api.model.User
import org.springframework.data.mongodb.repository.MongoRepository

interface UserRepository :MongoRepository<User,String>{
}