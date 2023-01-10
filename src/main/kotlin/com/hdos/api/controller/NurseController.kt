package com.hdos.api.controller

import com.hdos.api.model.Role
import com.hdos.api.model.User
import com.hdos.api.services.UserService
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/api/v1/nurse")
class NurseController(private val userService: UserService) {

    @PostMapping("/create")
    fun createUser(@RequestBody user:User): ResponseEntity<User>{
//        var user = User(
//            name = "Nurse 1",
//            email = "nurse@x.com",
//            authCode = "900",
//            createdAt = "mcjsndmkjs",
//            role = Role.Patient
//        )
        var newUser = userService.userRepository.save(user)
        return ResponseEntity.ok(newUser)
    }

//    @GetMapping("/get/{email}")
//    fun getUser(@PathVariable("email") email:String): ResponseEntity<User> {
//        println(email)
////        val user = userService.findByEMail(email)
//        return ResponseEntity.ok(user)
//    }
}