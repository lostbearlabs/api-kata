package com.lostbearlabs.springapi

import org.springframework.web.bind.annotation.*

@RestController
class JsonController {
    @PostMapping("/api/v1/hello-json", consumes=["application/json"], produces = ["application/json"])
    fun hello(@RequestBody payload: Payload): Answer {
        return if( payload.op=="+" ) {
            Answer("ok", payload.value1+payload.value2)
        } else {
            Answer("error", 0)
        }
    }
}

data class Payload (val op: String, val value1: Int, val value2: Int)
data class Answer(val stat: String, val result: Int)