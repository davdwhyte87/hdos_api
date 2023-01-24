package com.hdos.api

//import io.swagger.v3.oas.annotations.OpenAPIDefinition

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication


@SpringBootApplication

class HDOSApi

fun main(args: Array<String>) {
	runApplication<HDOSApi>(*args)
}
