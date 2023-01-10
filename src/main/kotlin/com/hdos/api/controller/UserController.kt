package com.hdos.api.controller

import com.hdos.api.model.Role
import com.hdos.api.model.User
import com.hdos.api.services.UserService
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/user")
class UserController(private val userService: UserService) {

    @GetMapping("create")
    fun createUser(){
        var user = User(
            name = "Jolofoo",
            email = "jofo@x.com",
            authCode = "900",
            createdAt = "msklmvsklfv",
            role = Role.Patient
        )
        userService.userRepository.save(user)
    }

    @GetMapping("/get/{email}")
    fun getUser(@PathVariable("email") email:String):ResponseEntity<User>{
        println(email)
        val user = userService.findByEMail(email)
        return ResponseEntity.ok(user)
    }
}