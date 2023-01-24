package com.hdos.api.controller

import com.hdos.api.model.Role

import com.hdos.api.model.User
import com.hdos.api.model.req.ConfirmUserReq
import com.hdos.api.services.UserService
import org.springframework.boot.autoconfigure.security.oauth2.resource.OAuth2ResourceServerProperties.Jwt
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import java.time.LocalDate
import java.util.Date


@RestController
@RequestMapping("/user")
class UserController(private val userService: UserService) {

    // this helps create a new user
    @PostMapping("/create")
    fun createUser(@RequestBody user:User):ResponseEntity<User>{
        user.createdAt = LocalDate.now().toString()

        // generate code and hash the code and save in the db

        var newUser = userService.userRepository.save(user)
        return ResponseEntity.ok(newUser)
    }

    // confirm that the user is the owner of the email
    // by sending a code to the email
    @PostMapping("/confirm")
    fun confirmUser(@RequestBody reqBody:ConfirmUserReq):ResponseEntity<Boolean>{
        var user = userService.userRepository.findById(reqBody.id)
        var updatedUser = user.get()
        updatedUser.isConfirmed = true
        userService.userRepository.save(updatedUser)
        // check if the code that the user supplies is correct
        return ResponseEntity.ok(true)
    }

    @GetMapping("/get/{email}")
    fun getUser(@PathVariable("email") email:String):ResponseEntity<User>{
        println(email)
        val user = userService.findByEMail(email)
        return ResponseEntity.ok(user)
    }

    @PostMapping("/login")
    fun loginUser(){
        val jwt = Jwts.build()
    }
}