#[cfg(test)]
#[cfg(target_arch = "wasm32")]
mod wasm_node {
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn it_transpiles() {
        match postman2openapi::transpile(COLLECTION, "yaml") {
            Ok(oas) => assert_eq!(OPENAPI, oas),
            Err(_) => assert!(false),
        }
    }

    static COLLECTION: &'static str = r#"
{
	"info": {
		"_postman_id": "a786732c-6dcd-4e0d-8411-c58acb2644c6",
		"name": "Postman Echo",
		"description": "Postman Echo is service you can use to test your REST clients and make sample API calls. It provides endpoints for `GET`, `POST`, `PUT`, various auth mechanisms and other utility endpoints.\n\nThe documentation for the endpoints as well as example responses can be found at [https://postman-echo.com](https://postman-echo.com/?source=echo-collection-app-onboarding)",
		"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
	},
	"item": [
		{
			"name": "Request Methods",
			"item": [
				{
					"name": "GET Request",
					"event": [
						{
							"listen": "test",
							"script": {
								"id": "2298dced-107f-4b06-afe7-8f1835d4477f",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with request queries\", function () {",
									"    pm.response.to.have.jsonBody('args.foo1', 'bar1')",
									"        .and.have.jsonBody('args.foo2', 'bar2');",
									"});"
								],
								"type": "text/javascript"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/get?foo1=bar1&foo2=bar2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"get"
							],
							"query": [
								{
									"key": "foo1",
									"value": "bar1"
								},
								{
									"key": "foo2",
									"value": "bar2"
								}
							]
						},
						"description": "The HTTP `GET` request method is meant to retrieve data from a server. The data\nis identified by a unique URI (Uniform Resource Identifier). \n\nA `GET` request can pass parameters to the server using \"Query String \nParameters\". For example, in the following request,\n\n> http://example.com/hi/there?hand=wave\n\nThe parameter \"hand\" has the value \"wave\".\n\nThis endpoint echoes the HTTP headers, request parameters and the complete\nURI requested."
					},
					"response": [
						{
							"name": "GET Request Woops",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://postman-echo.com/get?foo1=bar1&foo2=bar2",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"get"
									],
									"query": [
										{
											"key": "foo1",
											"value": "bar1"
										},
										{
											"key": "foo2",
											"value": "bar2"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Content-Encoding",
									"value": "gzip"
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8"
								},
								{
									"key": "Date",
									"value": "Tue, 11 Jun 2019 10:43:13 GMT"
								},
								{
									"key": "ETag",
									"value": "W/\"161-aLhNcsGArlgLSKbxPqfBW3viHPI\""
								},
								{
									"key": "Server",
									"value": "nginx"
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3AGz-wblZgXE8FCDq7aJpx_tUgZUcG3Nsw.LdNEN8L0C7nGWkvGLwvdw6R2s6Syjr%2FzkvyevA8qR0c; Path=/; HttpOnly"
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding"
								},
								{
									"key": "Content-Length",
									"value": "249"
								},
								{
									"key": "Connection",
									"value": "keep-alive"
								}
							],
							"cookie": [],
							"body": "{\n    \"args\": {\n        \"foo1\": \"bar1\",\n        \"foo2\": \"bar2\"\n    },\n    \"headers\": {\n        \"x-forwarded-proto\": \"https\",\n        \"host\": \"postman-echo.com\",\n        \"accept\": \"*/*\",\n        \"accept-encoding\": \"gzip, deflate\",\n        \"cache-control\": \"no-cache\",\n        \"postman-token\": \"5c27cd7d-6b16-4e5a-a0ef-191c9a3a275f\",\n        \"user-agent\": \"PostmanRuntime/7.6.1\",\n        \"x-forwarded-port\": \"443\"\n    },\n    \"url\": \"https://postman-echo.com/get?foo1=bar1&foo2=bar2\"\n}"
						}
					]
				},
				{
					"name": "POST Raw Text",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with request body\", function () {",
									"    pm.response.to.have.jsonBody('data', ",
									"        'This is expected to be sent back as part of response body.');",
									"});"
								],
								"id": "2dae8ce6-048b-422a-8d5a-64f233c8f701"
							}
						}
					],
					"request": {
						"method": "POST",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "This is expected to be sent back as part of response body."
						},
						"url": {
							"raw": "https://postman-echo.com/post",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"post"
							]
						},
						"description": "The HTTP `POST` request method is meant to transfer data to a server \n(and elicit a response). What data is returned depends on the implementation\nof the server.\n\nA `POST` request can pass parameters to the server using \"Query String \nParameters\", as well as the Request Body. For example, in the following request,\n\n> POST /hi/there?hand=wave\n>\n> <request-body>\n\nThe parameter \"hand\" has the value \"wave\". The request body can be in multiple\nformats. These formats are defined by the MIME type of the request. The MIME \nType can be set using the ``Content-Type`` HTTP header. The most commonly used \nMIME types are:\n\n* `multipart/form-data`\n* `application/x-www-form-urlencoded`\n* `application/json`\n\nThis endpoint echoes the HTTP headers, request parameters, the contents of\nthe request body and the complete URI requested."
					},
					"response": []
				},
				{
					"name": "POST Form Data",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with form data\", function () {",
									"    pm.response.to.have.jsonBody('form.foo1', 'bar1')",
									"        .and.have.jsonBody('form.foo2', 'bar2');",
									"});"
								],
								"id": "68b130c4-e7d7-4454-8499-ddabf8f8e847"
							}
						}
					],
					"request": {
						"method": "POST",
						"header": [],
						"body": {
							"mode": "urlencoded",
							"urlencoded": [
								{
									"key": "foo1",
									"value": "bar1",
									"type": "text"
								},
								{
									"key": "foo2",
									"value": "bar2",
									"type": "text"
								}
							]
						},
						"url": {
							"raw": "https://postman-echo.com/post",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"post"
							]
						},
						"description": "The HTTP `POST` request method is meant to transfer data to a server \n(and elicit a response). What data is returned depends on the implementation\nof the server.\n\nA `POST` request can pass parameters to the server using \"Query String \nParameters\", as well as the Request Body. For example, in the following request,\n\n> POST /hi/there?hand=wave\n>\n> <request-body>\n\nThe parameter \"hand\" has the value \"wave\". The request body can be in multiple\nformats. These formats are defined by the MIME type of the request. The MIME \nType can be set using the ``Content-Type`` HTTP header. The most commonly used \nMIME types are:\n\n* `multipart/form-data`\n* `application/x-www-form-urlencoded`\n* `application/json`\n\nThis endpoint echoes the HTTP headers, request parameters, the contents of\nthe request body and the complete URI requested when data is sent as a form parameter."
					},
					"response": []
				},
				{
					"name": "PUT Request",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with form data\", function () {",
									"    pm.response.to.have.jsonBody('data', ",
									"        'This is expected to be sent back as part of response body.');",
									"});"
								],
								"id": "566f44e3-3771-4e10-a3b6-34833f7a4c9c"
							}
						}
					],
					"request": {
						"method": "PUT",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "This is expected to be sent back as part of response body."
						},
						"url": {
							"raw": "https://postman-echo.com/put",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"put"
							]
						},
						"description": "The HTTP `PUT` request method is similar to HTTP `POST`. It too is meant to \ntransfer data to a server (and elicit a response). What data is returned depends on the implementation\nof the server.\n\nA `PUT` request can pass parameters to the server using \"Query String \nParameters\", as well as the Request Body. For example, in the following \nraw HTTP request,\n\n> PUT /hi/there?hand=wave\n>\n> <request-body>\n\n\n"
					},
					"response": []
				},
				{
					"name": "PATCH Request",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with form data\", function () {",
									"    pm.response.to.have.jsonBody('data', ",
									"        'This is expected to be sent back as part of response body.');",
									"});"
								],
								"id": "8bae2967-5d02-46ad-9e94-6663802014fb"
							}
						}
					],
					"request": {
						"method": "PATCH",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "This is expected to be sent back as part of response body."
						},
						"url": {
							"raw": "https://postman-echo.com/patch",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"patch"
							]
						},
						"description": "The HTTP `PATCH` method is used to update resources on a server. The exact\nuse of `PATCH` requests depends on the server in question. There are a number\nof server implementations which handle `PATCH` differently. Technically, \n`PATCH` supports both Query String parameters and a Request Body.\n\nThis endpoint accepts an HTTP `PATCH` request and provides debug information\nsuch as the HTTP headers, Query String arguments, and the Request Body."
					},
					"response": []
				},
				{
					"name": "DELETE Request",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json with form data\", function () {",
									"    pm.response.to.have.jsonBody('data', ",
									"        'This is expected to be sent back as part of response body.');",
									"});"
								],
								"id": "9519873b-58fa-46ae-9419-7772660bbc92"
							}
						}
					],
					"request": {
						"method": "DELETE",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "This is expected to be sent back as part of response body."
						},
						"url": {
							"raw": "https://postman-echo.com/delete",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"delete"
							]
						},
						"description": "The HTTP `DELETE` method is used to delete resources on a server. The exact\nuse of `DELETE` requests depends on the server implementation. In general, \n`DELETE` requests support both, Query String parameters as well as a Request \nBody.\n\nThis endpoint accepts an HTTP `DELETE` request and provides debug information\nsuch as the HTTP headers, Query String arguments, and the Request Body."
					},
					"response": []
				}
			],
			"description": "HTTP has multiple request \"verbs\", such as `GET`, `PUT`, `POST`, `DELETE`,\n`PATCH`, `HEAD`, etc. \n\nAn HTTP Method (verb) defines how a request should be interpreted by a server. \nThe endpoints in this section demonstrate various HTTP Verbs. Postman supports \nall the HTTP Verbs, including some rarely used ones, such as `PROPFIND`, `UNLINK`, \netc.\n\nFor details about HTTP Verbs, refer to [RFC 2616](http://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html#sec9)\n",
			"protocolProfileBehavior": {}
		},
		{
			"name": "Headers",
			"item": [
				{
					"name": "Request Headers",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"// we check to ensire that the headers we sent as request is returned as part of",
									"// the response body. if you change the request headers, make sure to add them",
									"// to the tests here",
									"pm.test(\"sample request header should be returned in response body\", function () {",
									"    pm.response.to.have.jsonBody('headers.my-sample-header', 'Lorem ipsum dolor sit amet');",
									"});"
								],
								"id": "43c03e2f-5798-4337-b9e9-4c0fecdae553"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [
							{
								"key": "my-sample-header",
								"value": "Lorem ipsum dolor sit amet"
							}
						],
						"url": {
							"raw": "https://postman-echo.com/headers",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"headers"
							]
						},
						"description": "A `GET` request to this endpoint returns the list of all request headers as part of the response JSON.\nIn Postman, sending your own set of headers through the [Headers tab](https://www.getpostman.com/docs/requests#headers?source=echo-collection-app-onboarding) will reveal the headers as part of the response."
					},
					"response": [
						{
							"name": "my-sample-header",
							"originalRequest": {
								"method": "GET",
								"header": [
									{
										"key": "my-sample-header",
										"value": "Lorem ipsum dolor sit amet"
									}
								],
								"url": {
									"raw": "https://echo.getpostman.com/headers",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"headers"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "342",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 31 Mar 2016 11:14:00 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"hostOnly": true,
									"httpOnly": true,
									"domain": "echo.getpostman.com",
									"path": "/",
									"secure": false,
									"session": true,
									"value": "s%3A9stja5zKmIILxq9Jvtha7Lp9LIz1VIdK.Vp8MHC%2BEUJe4ICZPXn2JAoXaV2bTgtoQd%2B3XJLNr51Y",
									"key": "sails.sid"
								}
							],
							"body": "{\"headers\":{\"host\":\"echo.getpostman.com\",\"accept\":\"*/*\",\"accept-encoding\":\"gzip, deflate, sdch\",\"accept-language\":\"en-US,en;q=0.8\",\"cache-control\":\"no-cache\",\"my-sample-header\":\"Lorem ipsum dolor sit amet\",\"postman-token\":\"3c8ea80b-f599-fba6-e0b4-a0910440e7b6\",\"user-agent\":\"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.110 Safari/537.36\",\"x-forwarded-port\":\"443\",\"x-forwarded-proto\":\"https\"}}"
						}
					]
				},
				{
					"name": "Response Headers",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has the headers sent as part of request query\", function () {",
									"    pm.response.to.have.header('foo1', 'bar1')",
									"        .and.have.header('foo2', 'bar2');",
									"});",
									"",
									"pm.test(\"sample request param should be returned in response body\", function () {",
									"    pm.response.to.have.jsonBody('foo1', 'bar1')",
									"        .and.have.jsonBody('foo2', 'bar2');",
									"});"
								],
								"id": "d3cb36b9-6d09-49fd-b7e2-bf36bcb670b0"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/response-headers?foo1=bar1&foo2=bar2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"response-headers"
							],
							"query": [
								{
									"key": "foo1",
									"value": "bar1"
								},
								{
									"key": "foo2",
									"value": "bar2"
								}
							]
						},
						"description": "This endpoint causes the server to send custom set of response headers. Providing header values as part of the URL parameters of a `GET` request to this endpoint returns the same as part of response header.\n\nTo send your own set of headers, simply add or replace the the URL parameters with your own set."
					},
					"response": [
						{
							"name": "Response headers",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://echo.getpostman.com/response-headers?Content-Type=text/html&test=response_headers",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"response-headers"
									],
									"query": [
										{
											"key": "Content-Type",
											"value": "text/html"
										},
										{
											"key": "test",
											"value": "response_headers"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "html",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "71",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "text/html; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 31 Mar 2016 11:14:18 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								},
								{
									"key": "test",
									"value": "response_headers",
									"name": "test",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"hostOnly": true,
									"httpOnly": true,
									"domain": "echo.getpostman.com",
									"path": "/",
									"secure": false,
									"session": true,
									"value": "s%3A9stja5zKmIILxq9Jvtha7Lp9LIz1VIdK.Vp8MHC%2BEUJe4ICZPXn2JAoXaV2bTgtoQd%2B3XJLNr51Y",
									"key": "sails.sid"
								}
							],
							"body": "{\"Content-Type\":\"text/html\",\"test\":\"response_headers\"}"
						}
					]
				}
			],
			"description": "The following set of endpoints allow one to see the headers being sent as part of a request and to get a custom set of headers as part of response.\n\nHTTP header fields provide required information about the request or response, or about the object sent in the message body. Both request headers and response headers can be controlled using these endpoints.",
			"protocolProfileBehavior": {}
		},
		{
			"name": "Authentication Methods",
			"item": [
				{
					"name": "Basic Auth",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json saying 'authenticated'\", function () {",
									"    pm.response.to.have.jsonBody('authenticated', true);",
									"});"
								],
								"id": "bc73a712-e124-4335-8634-58ebfcf9d85d"
							}
						}
					],
					"request": {
						"auth": {
							"type": "basic",
							"basic": [
								{
									"key": "username",
									"value": "postman",
									"type": "string"
								},
								{
									"key": "password",
									"value": "password",
									"type": "string"
								},
								{
									"key": "showPassword",
									"value": false,
									"type": "boolean"
								}
							]
						},
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/basic-auth",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"basic-auth"
							]
						},
						"description": "This endpoint simulates a **basic-auth** protected endpoint. \nThe endpoint accepts a default username and password and returns a status code of `200 ok` only if the same is provided. \nOtherwise it will return a status code `401 unauthorized`.\n\n> Username: `postman`\n> \n> Password: `password`\n\nTo use this endpoint, send a request with the header `Authorization: Basic cG9zdG1hbjpwYXNzd29yZA==`. \nThe cryptic latter half of the header value is a base64 encoded concatenation of the default username and password. \nUsing Postman, to send this request, you can simply fill in the username and password in the \"Authorization\" tab and Postman will do the rest for you.\n\nTo know more about basic authentication, refer to the [Basic Access Authentication](https://en.wikipedia.org/wiki/Basic_access_authentication) wikipedia article.\nThe article on [authentication helpers](https://www.getpostman.com/docs/helpers#basic-auth?source=echo-collection-app-onboarding) elaborates how to use the same within the Postman app."
					},
					"response": [
						{
							"name": "200",
							"originalRequest": {
								"header": [],
								"url": {
									"raw": ""
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "42",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Sat, 31 Oct 2015 06:38:25 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"authenticated\":true}"
						}
					]
				},
				{
					"name": "DigestAuth Success",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"tests[\"response code is 200\"] = responseCode.code === 200;",
									"tests[\"body contains authenticated\"] = responseBody.has(\"authenticated\");"
								],
								"id": "075fb2c3-36f7-4cf7-bc89-2e5049c2a1fc"
							}
						}
					],
					"request": {
						"auth": {
							"type": "digest",
							"digest": [
								{
									"key": "algorithm",
									"value": "MD5",
									"type": "string"
								},
								{
									"key": "username",
									"value": "postman",
									"type": "string"
								},
								{
									"key": "realm",
									"value": "{{echo_digest_realm}}",
									"type": "string"
								},
								{
									"key": "password",
									"value": "password",
									"type": "string"
								},
								{
									"key": "nonce",
									"value": "{{echo_digest_nonce}}",
									"type": "string"
								},
								{
									"key": "nonceCount",
									"value": "",
									"type": "string"
								},
								{
									"key": "clientNonce",
									"value": "",
									"type": "string"
								},
								{
									"key": "opaque",
									"value": "",
									"type": "string"
								},
								{
									"key": "qop",
									"value": "",
									"type": "string"
								}
							]
						},
						"method": "GET",
						"header": [
							{
								"key": "Authorization",
								"value": "Digest username=\"postman\", realm=\"Users\", nonce=\"ni1LiL0O37PRRhofWdCLmwFsnEtH1lew\", uri=\"/digest-auth\", response=\"254679099562cf07df9b6f5d8d15db44\", opaque=\"\""
							}
						],
						"url": {
							"raw": "https://postman-echo.com/digest-auth",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"digest-auth"
							]
						},
						"description": "This endpoint sends a hashed Digest Authorization header to gain access to a valid `200 Ok` response code. In Postman, it uses the stored [global variables](https://www.getpostman.com/docs/environments#gloval-variables?source=echo-collection-app-onboarding), `echo_digest_realm` and `echo_digest_nonce`, to generate the hashed authorisation header.\n\nWithin Postman, for this request to successfully authenticate, running the previous request \"DigestAuth Request\" stores the relevant information within the global variables."
					},
					"response": [
						{
							"name": "200",
							"originalRequest": {
								"header": [],
								"url": {
									"raw": ""
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "42",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 29 Oct 2015 06:17:51 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"authenticated\":true}"
						}
					]
				},
				{
					"name": "Hawk Auth",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json saying passed 'status'\", function () {",
									"    pm.response.to.have.jsonBody('status', 'pass');",
									"});"
								],
								"id": "9aa00a83-1844-4440-a99c-abe39c02d93b"
							}
						}
					],
					"request": {
						"auth": {
							"type": "hawk",
							"hawk": [
								{
									"key": "authId",
									"value": "dh37fgj492je",
									"type": "string"
								},
								{
									"key": "authKey",
									"value": "werxhqb98rpaxn39848xrunpaw3489ruxnpa98w4rxn",
									"type": "string"
								},
								{
									"key": "algorithm",
									"value": "sha256",
									"type": "string"
								},
								{
									"key": "user",
									"value": "",
									"type": "string"
								},
								{
									"key": "nonce",
									"value": "RZKGNz",
									"type": "string"
								},
								{
									"key": "timestamp",
									"value": "",
									"type": "string"
								}
							]
						},
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/auth/hawk",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"auth",
								"hawk"
							]
						},
						"description": "This endpoint is a Hawk Authentication protected endpoint. [Hawk authentication](https://github.com/hueniverse/hawk) is a widely used protocol for protecting API endpoints. One of Hawk's main goals is to enable HTTP authentication for services that do not use TLS (although it can be used in conjunction with TLS as well).\n\nIn order to use this endpoint, select the \"Hawk Auth\" helper inside Postman, and set the following values:\n\nHawk Auth ID: `dh37fgj492je`\n\nHawk Auth Key: `werxhqb98rpaxn39848xrunpaw3489ruxnpa98w4rxn`\n\nAlgorithm: `sha256`\n\nThe rest of the values are optional, and can be left blank. Hitting send should give you a response with a status code of 200 OK."
					},
					"response": [
						{
							"name": "Success",
							"originalRequest": {
								"method": "GET",
								"header": [
									{
										"key": "Authorization",
										"type": "text",
										"name": "Authorization",
										"value": "Hawk id=\"dh37fgj492je\", ts=\"1459422734\", nonce=\"XiwiCU\", mac=\"KzMHk67BYCC9zZqRy5hRdWFEFLHX5bNlRWGdmOAWKp0=\""
									}
								],
								"url": {
									"raw": "https://echo.getpostman.com/auth/hawk",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"auth",
										"hawk"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 31 Mar 2016 11:12:16 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Server-Authorization",
									"value": "Hawk mac=\"vRrUzDdcHu2NaNts/r4zg2xmXMdX8wPiTGTM398BDRg=\", hash=\"qmtflETMybaZiOQ2dLT17yiRunFT5OCIxZRZ0boQaiE=\"",
									"name": "Server-Authorization",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								},
								{
									"key": "transfer-encoding",
									"value": "chunked",
									"name": "transfer-encoding",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"hostOnly": false,
									"httpOnly": false,
									"domain": ".getpostman.com",
									"path": "/",
									"secure": false,
									"session": false,
									"value": "yes",
									"key": "getpostmanlogin"
								},
								{
									"expires": "Invalid Date",
									"hostOnly": false,
									"httpOnly": false,
									"domain": ".getpostman.com",
									"path": "/",
									"secure": false,
									"session": false,
									"value": "9f887f3b7f14b8c29ac4dc4109381b0b89a76e785c7b34251d6c8025b41b24013d2aa49f40e2deac19cbf0594dd984169455534d91ff98d4d1868d67ac857017629f137926e3a04a616bb83a2fb5ab9e6cbea9579ed5d5c1155d47545d72aad5be99f4abd0a7130805b3807d70cd507171dbe9d950d8e35a853f9ec075f5a767c95df4d57f7d521b66605b3bda3801700e26e651d1129c798b729ee3b91702d43ae64ab226c3f426893753def772c15442a7552dc84a3c773d6099a50b0a6af940b64c8176fedfcecd5fc31ccfc3bbc0124bfdaa0d62e4252d4aafb46a3c10963d12391e1fa97a1c0f19a636f57a3ac8cc35567d1cb6cb53b77f8adde3f6754a765596d7d00bdeb9acb5cc8d115e7c3f50ec3228e34d3e6c7464e9039b01868e03d10e9f87772397602453e9e91205de7b86021fad06eb26e69298e99ff1597a670faeb310f8c092041d544851de84f2bee89a92123da6eea286210524035c85361e2af42166a6",
									"key": "postman.sid"
								},
								{
									"expires": "Invalid Date",
									"hostOnly": true,
									"httpOnly": true,
									"domain": "echo.getpostman.com",
									"path": "/",
									"secure": false,
									"session": true,
									"value": "s%3AryJV7v-PE4PuTjBK6nH5XOynQ4atuATV.n17KcaLhVmV8TBHNLwdwXgGR7lmqs3i478WPlTbRgZ4",
									"key": "sails.sid"
								}
							],
							"body": "{\"status\":\"pass\",\"message\":\"Hawk Authentication successful\"}"
						}
					]
				},
				{
					"name": "OAuth1.0 (verify signature)",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response body has json saying passed 'status'\", function () {",
									"    pm.response.to.have.jsonBody('status', 'pass');",
									"});"
								],
								"id": "b81d641e-2b4b-49fe-bc55-1fb1b948a79d"
							}
						}
					],
					"request": {
						"auth": {
							"type": "oauth1",
							"oauth1": [
								{
									"key": "consumerKey",
									"value": "RKCGzna7bv9YD57c",
									"type": "string"
								},
								{
									"key": "consumerSecret",
									"value": "D+EdQ-gs$-%@2Nu7",
									"type": "string"
								},
								{
									"key": "token",
									"value": "",
									"type": "string"
								},
								{
									"key": "tokenSecret",
									"value": "",
									"type": "string"
								},
								{
									"key": "signatureMethod",
									"value": "HMAC-SHA1",
									"type": "string"
								},
								{
									"key": "timestamp",
									"value": "",
									"type": "string"
								},
								{
									"key": "nonce",
									"value": "",
									"type": "string"
								},
								{
									"key": "version",
									"value": "",
									"type": "string"
								},
								{
									"key": "realm",
									"value": "",
									"type": "string"
								},
								{
									"key": "addParamsToHeader",
									"value": true,
									"type": "boolean"
								},
								{
									"key": "addEmptyParamsToSign",
									"value": false,
									"type": "boolean"
								}
							]
						},
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/oauth1",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"oauth1"
							]
						},
						"description": "OAuth1.0a is a specification that defines a protocol that can be used by one\nservice to access \"protected\" resources (endpoints) on another service. A\nmajor part of OAuth1.0 is HTTP Request Signing. This endpoint allows you to \ncheck whether the request calculation works properly in the client. \n\nThe endpoint supports the HTTP ``Authorization`` header. In case the signature\nverification fails, the endpoint provides the four debug values,\n\n* ``base_uri``\n* ``normalized_param_string``\n* ``base_string``\n* ``signing_key``\n\nFor more details about these parameters, check the [OAuth1.0a Specification](http://oauth.net/core/1.0a/)\n\nIn order to use this endpoint, you can set the following values:\n\n> Consumer Key: ``RKCGzna7bv9YD57c``\n>\n> Consumer Secret: ``D+EdQ-gs$-%@2Nu7``\n\nIf you are using Postman, also check the \"Add params to header\" and \n\"Auto add parameters\" boxes."
					},
					"response": [
						{
							"name": "401",
							"originalRequest": {
								"method": "GET",
								"header": [
									{
										"key": "Authorization",
										"type": "text",
										"name": "Authorization",
										"value": "OAuth oauth_consumer_key=\"RKCGzna7bv9YD57c_wrong\",oauth_signature_method=\"HMAC-SHA1\",oauth_timestamp=\"1472121295\",oauth_nonce=\"8LTsU2\",oauth_version=\"1.0\",oauth_signature=\"tSUexpY%2B7EhSY7cFXiFN5EMx2zw%3D\""
									}
								],
								"url": {
									"raw": "https://echo.getpostman.com/oauth1",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"oauth1"
									]
								}
							},
							"status": "Unauthorized",
							"code": 401,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "536",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 25 Aug 2016 10:34:55 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"218-SGnurnTsu5qV5cCYWxsJlg\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.8.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"status\":\"fail\",\"message\":\"HMAC-SHA1 verification failed\",\"base_uri\":\"https://echo.getpostman.com/oauth1\",\"normalized_param_string\":\"oauth_consumer_key=RKCGzna7bv9YD57c_wrong&oauth_nonce=8LTsU2&oauth_signature_method=HMAC-SHA1&oauth_timestamp=1472121295&oauth_version=1.0\",\"base_string\":\"GET&https%3A%2F%2Fecho.getpostman.com%2Foauth1&oauth_consumer_key%3DRKCGzna7bv9YD57c_wrong%26oauth_nonce%3D8LTsU2%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1472121295%26oauth_version%3D1.0\",\"signing_key\":\"D%2BEdQ-gs%24-%25%402Nu7&\"}"
						},
						{
							"name": "200",
							"originalRequest": {
								"method": "GET",
								"header": [
									{
										"key": "Authorization",
										"name": "Authorization",
										"value": "OAuth oauth_consumer_key=\"RKCGzna7bv9YD57c\",oauth_signature_method=\"HMAC-SHA1\",oauth_timestamp=\"1472121261\",oauth_nonce=\"ki0RQW\",oauth_version=\"1.0\",oauth_signature=\"s0rK92Myxx7ceUBVzlMaxiiXU00%3D\""
									}
								],
								"url": {
									"raw": "https://echo.getpostman.com/oauth1",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"oauth1"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "95",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 25 Aug 2016 10:34:23 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"4e-Cq3UhvpVSyl6R6204lPVIA\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.8.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"status\":\"pass\",\"message\":\"OAuth-1.0a signature verification was successful\"}"
						}
					]
				}
			],
			"protocolProfileBehavior": {}
		},
		{
			"name": "Cookie Manipulation",
			"item": [
				{
					"name": "Set Cookies",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test('response should be 200 or 302', function () {",
									"    pm.expect(pm.response.code).to.be.oneOf([200, 302]);",
									"});",
									"",
									"pm.test('the \"foo1\" cookie has correct value', function () {",
									"    pm.expect(pm.cookies.toObject()).to.have.property('foo1', 'bar1');",
									"});",
									"",
									"pm.test('the \"foo2\" cookie has correct value', function () {",
									"    pm.expect(pm.cookies.toObject()).to.have.property('foo2', 'bar2');",
									"});",
									"",
									"// response code could be either a 200 or a redirection based on the settings of ",
									"// the http client. hence we need to handle both cases",
									"pm.test('response body should be valid', function () {",
									"    if (pm.response.code === 200) {",
									"        pm.response.to.have.jsonBody('cookies.foo1', 'bar1')",
									"            .and.have.jsonBody('cookies.foo2', 'bar2');",
									"    }",
									"    else {",
									"        pm.response.to.have.body('Found. Redirecting to /cookies');",
									"    }",
									"});"
								],
								"id": "ca4047f2-e141-4a62-b0b6-979b7965fca9"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/cookies/set?foo1=bar1&foo2=bar2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"cookies",
								"set"
							],
							"query": [
								{
									"key": "foo1",
									"value": "bar1"
								},
								{
									"key": "foo2",
									"value": "bar2"
								}
							]
						},
						"description": "The cookie setter endpoint accepts a list of cookies and their values as part of URL parameters of a `GET` request. These cookies are saved and can be subsequently retrieved or deleted. The response of this request returns a JSON with all cookies listed.\n\nTo set your own set of cookies, simply replace the URL parameters \"foo1=bar1&foo2=bar2\" with your own set of key-value pairs."
					},
					"response": [
						{
							"name": "Cookies",
							"originalRequest": {
								"header": [],
								"url": {
									"raw": ""
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "51",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 29 Oct 2015 06:15:28 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"cookies\":{\"foo1\":\"bar\",\"foo2\":\"bar\"}}"
						}
					]
				},
				{
					"name": "Get Cookies",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test('the \"foo1\" cookie has correct value', function () {",
									"    pm.expect(pm.cookies.toObject()).to.have.property('foo1', 'bar1');",
									"});",
									"",
									"pm.test('the \"foo2\" cookie has correct value', function () {",
									"    pm.expect(pm.cookies.toObject()).to.have.property('foo2', 'bar2');",
									"});",
									"",
									"pm.test('response body should be valid', function () {",
									"    pm.response.to.have.jsonBody('cookies.foo1', 'bar1');",
									"    pm.response.to.have.jsonBody('cookies.foo2', 'bar2');",
									"});"
								],
								"id": "8f45e735-9e30-48ab-938d-b020dc8f76e7"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/cookies",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"cookies"
							]
						},
						"description": "Use this endpoint to get a list of all cookies that are stored with respect to this domain. Whatever key-value pairs that has been previously set by calling the \"Set Cookies\" endpoint, will be returned as response JSON."
					},
					"response": [
						{
							"name": "Cookies",
							"originalRequest": {
								"header": [],
								"url": {
									"raw": ""
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "46",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 29 Oct 2015 06:16:29 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"cookies\":{\"foo2\":\"bar\"}}"
						}
					]
				},
				{
					"name": "Delete Cookies",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test('response should be 200 or 302', function () {",
									"    pm.expect(pm.response.code).to.be.oneOf([200, 302]);",
									"});",
									"",
									"pm.test('the \"foo1\" cookie should not be present', function () {",
									"    pm.expect(pm.cookies.toObject()).to.not.have.property('foo1');",
									"});",
									"",
									"pm.test('the \"foo2\" cookie should not be present', function () {",
									"    pm.expect(pm.cookies.toObject()).to.not.have.property('foo2');",
									"});",
									"",
									"// response code could be either a 200 or a redirection based on the settings of ",
									"// the http client. hence we need to handle both cases",
									"pm.test('response body should be valid', function () {",
									"    if (pm.response.code === 200) {",
									"        pm.response.to.not.have.jsonBody('cookies.foo1');",
									"        pm.response.to.not.have.jsonBody('cookies.foo2');",
									"    }",
									"    else {",
									"        pm.response.to.have.body('Found. Redirecting to /cookies');",
									"    }",
									"});"
								],
								"id": "df4bd414-401a-4134-817e-25a8404e6010"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/cookies/delete?foo1&foo2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"cookies",
								"delete"
							],
							"query": [
								{
									"key": "foo1",
									"value": null
								},
								{
									"key": "foo2",
									"value": null
								}
							]
						},
						"description": "One or more cookies that has been set for this domain can be deleted by providing the cookie names as part of the URL parameter. The response of this request is a JSON containing the list of currently set cookies."
					},
					"response": [
						{
							"name": "Cookies Response",
							"originalRequest": {
								"header": [],
								"url": {
									"raw": ""
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "46",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 29 Oct 2015 06:16:00 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [],
							"body": "{\"cookies\":{\"foo2\":\"bar\"}}"
						}
					]
				}
			],
			"description": "The cookie related endpoints allow one to get, set and delete simple cookies.\n\nCookies are small snippets of information that is stored in the browser and sent back to the server with every subsequent requests in order to store useful information between requests.\nIf you want to know more about cookies, read the [HTTP Cookie](https://en.wikipedia.org/wiki/HTTP_cookie) article on wikipedia.",
			"protocolProfileBehavior": {}
		},
		{
			"name": "Utilities",
			"item": [
				{
					"name": "Response Status Code",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has valid json body\", function () {",
									"    pm.response.to.have.jsonBody('status');",
									"});",
									"",
									"// additional sanity tests",
									"pm.test(\"status in response body must match the one in request\", function () {",
									"    pm.response.to.have.jsonBody('status', Number(_.get(pm.request, 'url.path[1]')));",
									"});"
								],
								"id": "40ff4603-fcc5-473e-b0c9-9c6ffd114ee2"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/status/200",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"status",
								"200"
							]
						},
						"description": "This endpoint allows one to instruct the server which status code to respond with.\n\nEvery response is accompanied by a status code. The status code provides a summary of the nature of response sent by the server. For example, a status code of `200` means everything is okay with the response and a code of `404` implies that the requested URL does not exist on server. \nA list of all valid HTTP status code can be found at the [List of Status Codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) wikipedia article. When using Postman, the response status code is described for easy reference.\n\nNote that if an invalid status code is requested to be sent, the server returns a status code of `400 Bad Request`."
					},
					"response": [
						{
							"name": "200",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://echo.getpostman.com/status/200",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"status",
										"200"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "javascript",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "14",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Thu, 31 Mar 2016 11:58:47 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"e-1056260003\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.6.2",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "X-Powered-By",
									"value": "Sails <sailsjs.org>",
									"name": "X-Powered-By",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"hostOnly": false,
									"httpOnly": false,
									"domain": ".getpostman.com",
									"path": "/",
									"secure": false,
									"session": false,
									"value": "yes",
									"key": "getpostmanlogin"
								},
								{
									"expires": "Invalid Date",
									"hostOnly": false,
									"httpOnly": false,
									"domain": ".getpostman.com",
									"path": "/",
									"secure": false,
									"session": false,
									"value": "df0c0256028d7ec4d641f766104a9571a8e249685bbc667d7cee030bbf44d3209495c70c03248e31e678a93812591d5e12187a8e99bf6bc5e80c40903f6ff6226938f24e413c0ffa613a7372064ec44a8594e8d3ede6945e34394f369573feeebc4a73a3e24b8c9ac18a53704addb5fd3f71f1ede488ff551feb059e9c1fb208164814e45e0312c4df8ea6e83c26702f42ae634c6afbe82d57c857bbf5598b5527961c1c28688dc2580070a4389f0cf4ec0a179b5b9c11b2ecbaa5460d374065bf5c7a3add9505df0fa89acb9f227f05ed2d4c6b58c39d6d728bd49f6f323ae67d4a75882aa7682f5d6fc5b981ba411d94aa93970bfaefa1953a73e440d50d012b5f288975c888e2345ee7777e746fb5aed3a7b2dbc087c6456621aa78c24a3c17c5f96cf59844933249a352f631e2008cffac6faf06d0e253dcc01cf0067bf56c1fbc5ed61fec1861b60c5accf35ffc2e56154a113004fa1db9d7171c3af8fc063918554092f5",
									"key": "postman.sid"
								},
								{
									"expires": "Invalid Date",
									"hostOnly": false,
									"httpOnly": false,
									"domain": ".echo.getpostman.com",
									"path": "/",
									"secure": false,
									"session": false,
									"value": "GA1.3.1703443399.1459422978",
									"key": "_ga"
								},
								{
									"expires": "Invalid Date",
									"hostOnly": true,
									"httpOnly": true,
									"domain": "echo.getpostman.com",
									"path": "/",
									"secure": false,
									"session": true,
									"value": "s%3AvuHU0EKeDbyNjVrEc7U30dMPzVu8CRaD.GOV1H9olcVzXqrwqP%2BC%2B6MVj2UczXivcN00jgPoDYfs",
									"key": "sails.sid"
								}
							],
							"body": "{\"status\":200}"
						}
					]
				},
				{
					"name": "Streamed Response",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has chunked transfer encoding header\", function () {",
									"    pm.response.to.have.header('transfer-encoding', 'chunked');",
									"});"
								],
								"id": "c12ab518-9d31-49d6-a923-5c843de312f5"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/stream/5",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"stream",
								"5"
							]
						},
						"description": "This endpoint allows one to recieve streaming http response using [chunked transfer encoding](https://en.wikipedia.org/wiki/Chunked_transfer_encoding) of a configurable length.\n\nA streaming response does not wait for the entire response to be generated on server before flushing it out. This implies that for a fairly large response, parts of it can be streamed to the requestee as and when it is generated on server. The client can then take actions of processing this partially received data."
					},
					"response": []
				},
				{
					"name": "Delay Response",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"var _ = require('lodash');",
									"",
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has valid json body\", function () {",
									"    pm.response.to.have.jsonBody('delay');",
									"});",
									"",
									"// additional sanity tests",
									"pm.test(\"request must have a valid delay\", function () {",
									"    pm.expect(Number(_.get(pm.request, 'url.path[1]'))).to.be.above(0).and.below(10);",
									"});",
									"pm.test(\"response should take more time than the delay specified\", function () {",
									"    pm.expect(pm.response.responseTime).to.be.above(Number(_.get(pm.request, 'url.path[1]')));",
									"});"
								],
								"id": "2c2dee90-f945-4729-88ca-0e1977318e22"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/delay/2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"delay",
								"2"
							]
						},
						"description": "Using this endpoint one can configure how long it takes for the server to come back with a response. Appending a number to the URL defines the time (in seconds) the server will wait before responding.\n\nNote that a maximum delay of 10 seconds is accepted by the server."
					},
					"response": [
						{
							"name": "success-response",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://echo.getpostman.com/delay/3",
									"protocol": "https",
									"host": [
										"echo",
										"getpostman",
										"com"
									],
									"path": [
										"delay",
										"3"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "13",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Mon, 02 Jan 2017 09:19:03 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"d-t/L/D5c0SDl+MoXtKdSVOg\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "echo.getpostman.com",
									"path": "/",
									"secure": false,
									"value": "s%3AYjUiFBtGiJVL2a-qzZQZ1DFlAMhgXN9O.WaAjRUV0OteZxwmhbNibuB7VKse068JJIh6PwLQUKmQ",
									"key": "sails.sid"
								}
							],
							"body": "{\"delay\":\"3\"}"
						}
					]
				},
				{
					"name": "Get UTF8 Encoded Response",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has charset in content-type header\", function () {",
									"    pm.response.to.have.header('content-type', 'text/html; charset=utf-8');",
									"});",
									"",
									"pm.test(\"response has chunked transfer encoding header\", function () {",
									"    pm.response.to.have.header('transfer-encoding', 'chunked');",
									"});"
								],
								"id": "d75f471a-84e5-4bcb-9cad-1379d1d4297b"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/encoding/utf8",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"encoding",
								"utf8"
							]
						},
						"description": "If a response of an endpoint requires to send data beyond the basic English / ASCII character set, the `charset` parameter in the `Content-Type` response header defines the character encoding policy.\n\nThis endpoint returns an `UTF8` character encoded response body with text in various languages such as Greek, Latin, East Asian, etc. Postman can interpret the character encoding and use appropriate methods to display the character set in responses."
					},
					"response": []
				},
				{
					"name": "GZip Compressed Response",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has valid json body\", function () {",
									"    pm.response.to.have.jsonBody('gzipped', true);",
									"});",
									"",
									"pm.test('response headers should indicate valid content encoding', function  () {",
									"    pm.response.to.have.header('content-encoding', 'gzip')",
									"        .and.have.header('content-type', 'application/json');",
									"});",
									"",
									"pm.test('requesting http client should accept compressed response', function () {",
									"    pm.expect(pm.response.json()).to.have.nested.property('headers.accept-encoding')",
									"        .and.to.match(/.*gzip.*/);",
									"});"
								],
								"id": "f9ef487e-c22d-49bd-b85c-a85f7e7cbb8a"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/gzip",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"gzip"
							]
						},
						"description": "This endpoint returns the response using [gzip compression algoritm](https://en.wikipedia.org/wiki/Gzip).\nThe uncompressed response is a JSON string containing the details of the request sent by the client. For this endpoint to work, one should request with `Accept-encoding` header containing `gzip` as part of its value. Postman supports gzip, deflate and SDCH decoding and automatically sends them as part of the request.\n\nHTTP Compression allows the server to send responses in a compressed format, which is uncompressed by the client before processing. This reduces network bandwidth consumption at the cost of increase in CPU usage.\nTo know more about this, refer the [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression) wikipedia article."
					},
					"response": []
				},
				{
					"name": "Deflate Compressed Response",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has valid json body\", function () {",
									"    pm.response.to.have.jsonBody('deflated', true);",
									"});",
									"",
									"pm.test('response headers should indicate valid content encoding', function  () {",
									"    pm.response.to.have.header('content-encoding', 'deflate')",
									"        .and.have.header('content-type', 'application/json');",
									"});",
									"",
									"pm.test('requesting http client should accept compressed response', function () {",
									"    pm.expect(pm.response.json()).to.have.nested.property('headers.accept-encoding')",
									"        .and.to.match(/.*deflate.*/);",
									"});"
								],
								"id": "d91c7c4d-1a3f-4565-ae00-0fc51fb5b7e2"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/deflate",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"deflate"
							]
						},
						"description": "This endpoint returns the response using [deflate compression algoritm](https://en.wikipedia.org/wiki/DEFLATE). \nThe uncompressed response is a JSON string containing the details of the request sent by the client. For this endpoint to work, one should request with `Accept-encoding` header containing `deflate` as part of its value. Postman supports gzip, deflate and SDCH decoding and automatically sends them as part of the request.\n\nHTTP Compression allows the server to send responses in a compressed format, which is uncompressed by the client before processing. This reduces network bandwidth consumption at the cost of increase in CPU usage.\nTo know more about this, refer the [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression) wikipedia article."
					},
					"response": []
				},
				{
					"name": "IP address in JSON format",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has valid json body\", function () {",
									"    pm.response.to.have.jsonBody('ip');",
									"});",
									"",
									"pm.test(\"response must return a valid ip address\", function () {",
									"    pm.expect(pm.response.json().ip).to",
									"        // a really gnarly regular expression to ensure that ip address is in correct format",
									"        .match(/^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/);",
									"});"
								],
								"id": "fdcf1496-1f92-4580-bea3-866a3a4ad175"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/ip",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"ip"
							]
						},
						"description": "A simple `GET` request to return the IP address of the source request in the following `JSON` format:\n\n```json\n{\n  ip: \"request-ip-address\"\n}\n```"
					},
					"response": []
				}
			],
			"protocolProfileBehavior": {}
		},
		{
			"name": "Utilities / Date and Time",
			"item": [
				{
					"name": "Current UTC time",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response time matches server response 'date' header\", function () {",
									"    pm.expect(pm.response.text()).to.eql(pm.response.headers.get('date'))",
									"});"
								],
								"id": "5dad209b-1402-4c3f-997e-aa24d7afe314"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/now",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"now"
							]
						},
						"description": "A simple `GET` request to `/time/now` to return the current timestamp as a UTC string.\n\n```\nFri, 04 Nov 2016 09:00:46 GMT\n```"
					},
					"response": [
						{
							"name": "time as text",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://postman-echo.com/time/now",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"time",
										"now"
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "html",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "49",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "text/html; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Wed, 11 Jan 2017 10:27:12 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"1d-2jJhkzratfVX9VZ0+raHbw\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3A2lT3TO7qS1tadeSAp4axl-NcXG9CV6Rf.HGqLY%2FlKEKY4fgCLePaAZs3tCHp%2Bglf7ZOJYlonGeig; Path=/; HttpOnly",
									"name": "set-cookie",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "postman-echo.com",
									"path": "/",
									"secure": false,
									"value": "s%3A2lT3TO7qS1tadeSAp4axl-NcXG9CV6Rf.HGqLY%2FlKEKY4fgCLePaAZs3tCHp%2Bglf7ZOJYlonGeig",
									"key": "sails.sid"
								}
							],
							"body": "Wed, 11 Jan 2017 10:27:12 GMT"
						}
					]
				},
				{
					"name": "Timestamp validity",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say whether request query time is valid\", function () {",
									"    pm.response.to.have.jsonBody('valid', true);",
									"});"
								],
								"id": "2906a7f0-5dbf-4583-bf1a-7ba870130f9a"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/valid?timestamp=2016-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"valid"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/valid` to determine the validity of the timestamp, (current by default).\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a valid key to indicate the result. The response code is `200`.\n\n```\n{\n  valid: true/false\n}\n```"
					},
					"response": [
						{
							"name": "Valid Timestamp",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://postman-echo.com/time/valid?timestamp=2016-10-10",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"time",
										"valid"
									],
									"query": [
										{
											"key": "timestamp",
											"value": "2016-10-10"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "14",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Wed, 11 Jan 2017 10:27:33 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"e-OYN7L87J1Ba9oy5mJE2kcA\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3AdDGZPe1CZw4mAxGVCHr6RfCADCAwquXa.F5MEm5LJad30JHrSwGGoyWLn2OAAGdvUM7kDtzNfdFI; Path=/; HttpOnly",
									"name": "set-cookie",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "postman-echo.com",
									"path": "/",
									"secure": false,
									"value": "s%3AdDGZPe1CZw4mAxGVCHr6RfCADCAwquXa.F5MEm5LJad30JHrSwGGoyWLn2OAAGdvUM7kDtzNfdFI",
									"key": "sails.sid"
								}
							],
							"body": "{\"valid\":true}"
						},
						{
							"name": "Invalid Timestamp",
							"originalRequest": {
								"method": "GET",
								"header": [],
								"url": {
									"raw": "https://postman-echo.com/time/valid?timestamp=2016-10-10",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"time",
										"valid"
									],
									"query": [
										{
											"key": "timestamp",
											"value": "2016-10-10"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Length",
									"value": "15",
									"name": "Content-Length",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Wed, 11 Jan 2017 10:27:53 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"f-/i9mO/upK91ZtL0BkKFGtw\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3ATNJaNxi2QCv4RPBb64sIZxQGN1h6IP3g.9sQVAijlsLsh0r7LgffxXa9k2we6UumPEVv%2Bsk4woLI; Path=/; HttpOnly",
									"name": "set-cookie",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "postman-echo.com",
									"path": "/",
									"secure": false,
									"value": "s%3ATNJaNxi2QCv4RPBb64sIZxQGN1h6IP3g.9sQVAijlsLsh0r7LgffxXa9k2we6UumPEVv%2Bsk4woLI",
									"key": "sails.sid"
								}
							],
							"body": "{\"valid\":false}"
						}
					]
				},
				{
					"name": "Format timestamp",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say whether request query time is valid\", function () {",
									"    pm.response.to.have.jsonBody('format', '20');",
									"});"
								],
								"id": "77f2a373-6cc0-481c-beaa-277abc8abea5"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/format?timestamp=2016-10-10&format=mm",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"format"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "format",
									"value": "mm"
								}
							]
						},
						"description": "A simple `GET` request to `/time/format` to convert the timestamp to any desired valid format.\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `format` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  format: \"formatted-timestamp\"\n}\n```"
					},
					"response": []
				},
				{
					"name": "Extract timestamp unit",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say whether request query unit is valid\", function () {",
									"    pm.response.to.have.jsonBody('unit', 1);",
									"});"
								],
								"id": "6184ff7f-d4f4-43d1-83ca-20b2fc0bdab3"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/unit?timestamp=2016-10-10&unit=day",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"unit"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "unit",
									"value": "day"
								}
							]
						},
						"description": "A simple `GET` request to `/time/unit` to extract the specified timestamp unit (as provided in the `unit` query parameter). The default unit returned is the `year`.\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `unit` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  unit: \"extracted-timestamp-unit\"\n}\n```"
					},
					"response": []
				},
				{
					"name": "Time addition",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should return the added years\", function () {",
									"    pm.response.to.have.jsonBody('sum', 'Sat Oct 10 2116 00:00:00 GMT+0000');",
									"});"
								],
								"id": "e9b565fe-cd3b-40c8-9a82-2a68c408cddb"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/add?timestamp=2016-10-10&years=100",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"add"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "years",
									"value": "100"
								}
							]
						},
						"description": "A simple `GET` request to `/time/add` to add units of time to the specified / current timestamp (as provided in the `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds` query parameters).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `sum` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  sum: \"sum of (provided / current) and provided timestamps\"\n}\n```"
					},
					"response": []
				},
				{
					"name": "Time subtraction",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should return the subtracted years\", function () {",
									"    pm.response.to.have.jsonBody('difference', 'Mon Oct 10 1966 00:00:00 GMT+0000');",
									"});"
								],
								"id": "df3a33b7-bfd7-42f0-b038-e351cd0c645d"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/subtract?timestamp=2016-10-10&years=50",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"subtract"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "years",
									"value": "50"
								}
							]
						},
						"description": "A simple `GET` request to `/time/subtract` to subtract units of time from the specified / current timestamp (as provided in the `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds` query parameters).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `difference` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  difference: \"difference between (provided / current) and provided timestamps\"\n}\n```"
					},
					"response": []
				},
				{
					"name": "Start of time",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should return the beginning of month\", function () {",
									"    pm.response.to.have.jsonBody('start', 'Sat Oct 01 2016 00:00:00 GMT+0000');",
									"});"
								],
								"id": "07d5bdb9-9885-49c7-96f7-abdd0b206b4a"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/start?timestamp=2016-10-10&unit=month",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"start"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "unit",
									"value": "month"
								}
							]
						},
						"description": "A simple `GET` request to `/time/start` to return a relative timstamp in the past from the specified / current timestamp (as provided in the `unit` query parameter).\n\nFor instance, if the `unit` has been specified as `month`, the returned timestamp would indicate the beginning of the current month. Similar results are returned for other units of time, like: `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds`\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `start` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  start: \"A timestamp from the past, depending on the `unit` specified\"\n}\n```"
					},
					"response": []
				},
				{
					"name": "Object representation",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should return the date components\", function () {",
									"    pm.expect(pm.response.json()).to.eql({",
									"        \"years\": 2016,",
									"        \"months\": 9,",
									"        \"date\": 10,",
									"        ",
									"        \"hours\": 0,",
									"        \"minutes\": 0,",
									"        \"seconds\": 0,",
									"        \"milliseconds\": 0",
									"    });",
									"});"
								],
								"id": "c61ef480-8379-447b-b9fa-7912d2ee2778"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/object?timestamp=2016-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"object"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/object` to return the current / provided timestamp as a JSON object.\n\nFor instance, if the `unit` has been specified as `month`, the returned timestamp would indicate the beginning of the current month. Similar results are returned for other units of time, like: `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds`\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  years: 2016,\n  months: 10,\n  days: 10,\n  hours: 23,\n  minutes: 34,\n  seconds: 20,\n  milliseconds: 980\n}\n```"
					},
					"response": []
				},
				{
					"name": "Before comparisons",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say timestamp is before target\", function () {",
									"    pm.response.to.have.jsonBody('before', true);",
									"});"
								],
								"id": "39edae06-6a91-4ddf-8de9-8d7f15ed908c"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/before?timestamp=2016-10-10&target=2017-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"before"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "target",
									"value": "2017-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/before` to check if the provided timestamps is before a comparison `target` (query parameter).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `before` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  before: true/false\n}\n```"
					},
					"response": []
				},
				{
					"name": "After comparisons",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say timestamp is not after target\", function () {",
									"    pm.response.to.have.jsonBody('after', false);",
									"});"
								],
								"id": "dd2c7702-4911-4a81-a8e9-cf3104fb6464"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/after?timestamp=2016-10-10&target=2017-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"after"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "target",
									"value": "2017-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/after` to check if the provided timestamps is after a comparison `target` (query parameter).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `after` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  after: true/false\n}\n```"
					},
					"response": []
				},
				{
					"name": "Between timestamps",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say timestamp is not between start and end\", function () {",
									"    pm.response.to.have.jsonBody('between', false);",
									"});"
								],
								"id": "4a3ba4bf-c4ae-41e5-9f4b-18ffbe6f4a76"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/between?timestamp=2016-10-10&start=2017-10-10&end=2019-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"between"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								},
								{
									"key": "start",
									"value": "2017-10-10"
								},
								{
									"key": "end",
									"value": "2019-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/between` to check if the provided timestamp is between a range specified by the `start` and `end` query parameters. A resolution limit can also be specified by the `unit` query parameter.\n\nFor instance, for a resolution `unit` of `month`,\n`2016-10-05` does lie between `2016-11-02` and `2016-09-01`.\n\nThis endpoint also accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `between` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  between: true/false\n}\n```"
					},
					"response": []
				},
				{
					"name": "Leap year check",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response json should say timestamp is within leap year\", function () {",
									"    pm.response.to.have.jsonBody('leap', true);",
									"});"
								],
								"id": "2a10ac4d-426c-4e79-9a61-efcb2b4b1a92"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/time/leap?timestamp=2016-10-10",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"time",
								"leap"
							],
							"query": [
								{
									"key": "timestamp",
									"value": "2016-10-10"
								}
							]
						},
						"description": "A simple `GET` request to `/time/leap` to check if the provided/current timestamp belongs to a leap year.\n\nThis endpoint also accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `leap` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  leap: true/false\n}\n```"
					},
					"response": []
				}
			],
			"description": "A set of `/time/*` mounted requests to perform date-time manipulations, among other operations.\n",
			"protocolProfileBehavior": {}
		},
		{
			"name": "Utilities / Postman Collection",
			"item": [
				{
					"name": "Transform collection from format v1 to v2",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has almost valid items\", function () {",
									"    pm.response.to.have.jsonBody('info.schema', 'https://schema.getpostman.com/json/collection/v2.0.0/collection.json')",
									"        .and.to.have.jsonBody('item[0].name', 'A simple GET request')",
									"        .and.to.have.jsonBody('item[1].name', 'A simple POST request');",
									"});"
								],
								"id": "ad900689-d7e4-48c4-b4ce-c8b493db6f79"
							}
						}
					],
					"request": {
						"method": "POST",
						"header": [
							{
								"key": "Content-Type",
								"value": "application/json"
							}
						],
						"body": {
							"mode": "raw",
							"raw": "{\n  \"id\": \"7875be4b-917d-4aff-8cc4-5606c36bf418\",\n  \"name\": \"Sample Postman Collection\",\n  \"description\": \"A sample collection to demonstrate collections as a set of related requests\",\n  \"order\": [\n    \"4d9134be-e8bf-4693-9cd7-1c0fc66ae739\",\n    \"141ba274-cc50-4377-a59c-e080066f375e\"\n  ],\n  \"folders\": [],\n  \"requests\": [\n    {\n      \"id\": \"4d9134be-e8bf-4693-9cd7-1c0fc66ae739\",\n      \"name\": \"A simple GET request\",\n      \"collectionId\": \"877b9dae-a50e-4152-9b89-870c37216f78\",\n      \"method\": \"GET\",\n      \"headers\": \"\",\n      \"data\": [],\n      \"rawModeData\": \"\",\n      \"tests\": \"tests['response code is 200'] = (responseCode.code === 200);\",\n      \"preRequestScript\": \"\",\n      \"url\": \"https://postman-echo.com/get?source=newman-sample-github-collection\"\n    },\n    {\n      \"id\": \"141ba274-cc50-4377-a59c-e080066f375e\",\n      \"name\": \"A simple POST request\",\n      \"collectionId\": \"877b9dae-a50e-4152-9b89-870c37216f78\",\n      \"method\": \"POST\",\n      \"headers\": \"Content-Type: text/plain\",\n      \"dataMode\": \"raw\",\n      \"data\": [],\n      \"rawModeData\": \"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\",\n      \"url\": \"https://postman-echo.com/post\"\n    }\n  ]\n}"
						},
						"url": {
							"raw": "https://postman-echo.com/transform/collection?from=1&to=2",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"transform",
								"collection"
							],
							"query": [
								{
									"key": "from",
									"value": "1"
								},
								{
									"key": "to",
									"value": "2"
								}
							]
						}
					},
					"response": [
						{
							"name": "Sample v2 Response",
							"originalRequest": {
								"method": "POST",
								"header": [
									{
										"key": "Content-Type",
										"value": "application/json",
										"description": "The mime type of this content"
									}
								],
								"body": {
									"mode": "raw",
									"raw": "{\n  \"id\": \"7875be4b-917d-4aff-8cc4-5606c36bf418\",\n  \"name\": \"Sample Postman Collection\",\n  \"description\": \"A sample collection to demonstrate collections as a set of related requests\",\n  \"order\": [\n    \"4d9134be-e8bf-4693-9cd7-1c0fc66ae739\",\n    \"141ba274-cc50-4377-a59c-e080066f375e\",\n    \"4511ca8b-0bc7-430f-b894-a7ec1036f322\"\n  ],\n  \"folders\": [],\n  \"requests\": [\n    {\n      \"id\": \"4d9134be-e8bf-4693-9cd7-1c0fc66ae739\",\n      \"name\": \"A simple GET request\",\n      \"collectionId\": \"877b9dae-a50e-4152-9b89-870c37216f78\",\n      \"method\": \"GET\",\n      \"headers\": \"\",\n      \"data\": [],\n      \"rawModeData\": \"\",\n      \"tests\": \"tests['response code is 200'] = (responseCode.code === 200);\",\n      \"preRequestScript\": \"\",\n      \"url\": \"https://postman-echo.com/get?source=newman-sample-github-collection\"\n    },\n    {\n      \"id\": \"141ba274-cc50-4377-a59c-e080066f375e\",\n      \"name\": \"A simple POST request\",\n      \"collectionId\": \"877b9dae-a50e-4152-9b89-870c37216f78\",\n      \"method\": \"POST\",\n      \"headers\": \"Content-Type: text/plain\",\n      \"dataMode\": \"raw\",\n      \"data\": [],\n      \"rawModeData\": \"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\",\n      \"url\": \"https://postman-echo.com/post\"\n    },\n    {\n      \"id\": \"4511ca8b-0bc7-430f-b894-a7ec1036f322\",\n      \"name\": \"A simple POST request with JSON body\",\n      \"collectionId\": \"877b9dae-a50e-4152-9b89-870c37216f78\",\n      \"method\": \"POST\",\n      \"headers\": \"Content-Type: application/json\",\n      \"dataMode\": \"raw\",\n      \"data\": [],\n      \"rawModeData\": \"{\\\"text\\\":\\\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\"}\",\n      \"url\": \"https://postman-echo.com/post\"\n    }\n  ]\n}"
								},
								"url": {
									"raw": "https://postman-echo.com/transform/collection?from=1&to=2",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"transform",
										"collection"
									],
									"query": [
										{
											"key": "from",
											"value": "1"
										},
										{
											"key": "to",
											"value": "2"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Wed, 11 Jan 2017 10:41:32 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"4cc-7P727Clhlrl9+b1/vneniw\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "X-HTTP-Method-Override, Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3AHtnQ1hlPxoj7wZahoNkcjN-aw9nQL0fc.KSyfLbEKhv1Lt3LvH13Ogjv9ENZgsBBSM6V8Y7TqVOU; Path=/; HttpOnly",
									"name": "set-cookie",
									"description": ""
								},
								{
									"key": "transfer-encoding",
									"value": "chunked",
									"name": "transfer-encoding",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "postman-echo.com",
									"path": "/",
									"secure": false,
									"value": "s%3AHtnQ1hlPxoj7wZahoNkcjN-aw9nQL0fc.KSyfLbEKhv1Lt3LvH13Ogjv9ENZgsBBSM6V8Y7TqVOU",
									"key": "sails.sid"
								}
							],
							"body": "{\"variables\":[],\"info\":{\"name\":\"Sample Postman Collection\",\"_postman_id\":\"7875be4b-917d-4aff-8cc4-5606c36bf418\",\"description\":\"A sample collection to demonstrate collections as a set of related requests\",\"schema\":\"https://schema.getpostman.com/json/collection/v2.0.0/collection.json\"},\"item\":[{\"name\":\"A simple GET request\",\"event\":[{\"listen\":\"test\",\"script\":{\"type\":\"text/javascript\",\"exec\":[\"tests['response code is 200'] = (responseCode.code === 200);\"]}}],\"request\":{\"url\":\"https://postman-echo.com/get?source=newman-sample-github-collection\",\"method\":\"GET\",\"header\":[],\"body\":{\"mode\":\"raw\",\"raw\":\"\"}},\"response\":[]},{\"name\":\"A simple POST request\",\"request\":{\"url\":\"https://postman-echo.com/post\",\"method\":\"POST\",\"header\":[{\"key\":\"Content-Type\",\"value\":\"text/plain\",\"description\":\"\"}],\"body\":{\"mode\":\"raw\",\"raw\":\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\"}},\"response\":[]},{\"name\":\"A simple POST request with JSON body\",\"request\":{\"url\":\"https://postman-echo.com/post\",\"method\":\"POST\",\"header\":[{\"key\":\"Content-Type\",\"value\":\"application/json\",\"description\":\"\"}],\"body\":{\"mode\":\"raw\",\"raw\":\"{\\\"text\\\":\\\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\"}\"}},\"response\":[]}]}"
						}
					]
				},
				{
					"name": "Transform collection from format v2 to v1",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"pm.test(\"response is ok\", function () {",
									"    pm.response.to.have.status(200);",
									"});",
									"",
									"pm.test(\"response has almost valid items\", function () {",
									"    pm.response.to.have.jsonBody('folders', [])",
									"        .and.to.have.jsonBody('order')",
									"        .and.to.have.jsonBody('requests[0].name', 'A simple GET request')",
									"        .and.to.have.jsonBody('requests[1].name', 'A simple POST request');",
									"});"
								],
								"id": "418d6f59-a86f-4f35-b80a-8c700a6dd56f"
							}
						}
					],
					"request": {
						"method": "POST",
						"header": [
							{
								"key": "Content-Type",
								"value": "application/json"
							}
						],
						"body": {
							"mode": "raw",
							"raw": "{\n  \"info\": {\n    \"name\": \"Sample Postman Collection\",\n    \"schema\": \"https://schema.getpostman.com/json/collection/v2.0.0/collection.json\",\n    \"description\": \"A sample collection to demonstrate collections as a set of related requests\"\n  },\n\n  \"item\": [{\n    \"name\": \"A simple GET request\",\n    \"request\": {\n      \"url\": \"https://postman-echo.com/get?source=newman-sample-github-collection\",\n      \"method\": \"GET\"\n    }\n  }, {\n    \"name\": \"A simple POST request\",\n    \"request\": {\n      \"url\": \"https://postman-echo.com/post\",\n      \"method\": \"POST\",\n      \"header\": [{\n        \"key\": \"Content-Type\",\n        \"value\": \"text/plain\"\n      }],\n      \"body\": {\n        \"mode\": \"raw\",\n        \"raw\": \"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\"\n      }\n    }\n  }]\n}"
						},
						"url": {
							"raw": "https://postman-echo.com/transform/collection?from=2&to=1",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"transform",
								"collection"
							],
							"query": [
								{
									"key": "from",
									"value": "2"
								},
								{
									"key": "to",
									"value": "1"
								}
							]
						}
					},
					"response": [
						{
							"name": "Sample v1 Response",
							"originalRequest": {
								"method": "POST",
								"header": [
									{
										"key": "Content-Type",
										"value": "application/json",
										"description": "The mime type of this content"
									}
								],
								"body": {
									"mode": "raw",
									"raw": "{\n  \"info\": {\n    \"name\": \"Sample Postman Collection\",\n    \"schema\": \"https://schema.getpostman.com/json/collection/v2.0.0/collection.json\",\n    \"description\": \"A sample collection to demonstrate collections as a set of related requests\"\n  },\n\n  \"item\": [{\n    \"name\": \"A simple GET request\",\n    \"event\": [{\n      \"listen\": \"test\",\n      \"script\": {\n        \"type\": \"text/javascript\",\n        \"exec\": [\"tests['response code is 200'] = (responseCode.code === 200);\"]\n      }\n    }],\n    \"request\": {\n      \"url\": \"https://postman-echo.com/get?source=newman-sample-github-collection\",\n      \"method\": \"GET\"\n    }\n  }, {\n    \"name\": \"A simple POST request\",\n    \"request\": {\n      \"url\": \"https://postman-echo.com/post\",\n      \"method\": \"POST\",\n      \"header\": [{\n        \"key\": \"Content-Type\",\n        \"value\": \"text/plain\"\n      }],\n      \"body\": {\n        \"mode\": \"raw\",\n        \"raw\": \"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\"\n      }\n    }\n  }, {\n    \"name\": \"A simple POST request with JSON body\",\n    \"request\": {\n      \"url\": \"https://postman-echo.com/post\",\n      \"method\": \"POST\",\n      \"header\": [{\n        \"key\": \"Content-Type\",\n        \"value\": \"application/json\"\n      }],\n      \"body\": {\n        \"mode\": \"raw\",\n        \"raw\": \"{\\\"text\\\":\\\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\"}\"\n      }\n    }\n  }]\n}"
								},
								"url": {
									"raw": "https://postman-echo.com/transform/collection?from=2&to=1",
									"protocol": "https",
									"host": [
										"postman-echo",
										"com"
									],
									"path": [
										"transform",
										"collection"
									],
									"query": [
										{
											"key": "from",
											"value": "2"
										},
										{
											"key": "to",
											"value": "1"
										}
									]
								}
							},
							"status": "OK",
							"code": 200,
							"_postman_previewlanguage": "json",
							"header": [
								{
									"key": "Access-Control-Allow-Credentials",
									"value": "",
									"name": "Access-Control-Allow-Credentials",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Headers",
									"value": "",
									"name": "Access-Control-Allow-Headers",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Methods",
									"value": "",
									"name": "Access-Control-Allow-Methods",
									"description": ""
								},
								{
									"key": "Access-Control-Allow-Origin",
									"value": "",
									"name": "Access-Control-Allow-Origin",
									"description": ""
								},
								{
									"key": "Access-Control-Expose-Headers",
									"value": "",
									"name": "Access-Control-Expose-Headers",
									"description": ""
								},
								{
									"key": "Connection",
									"value": "keep-alive",
									"name": "Connection",
									"description": ""
								},
								{
									"key": "Content-Encoding",
									"value": "gzip",
									"name": "Content-Encoding",
									"description": ""
								},
								{
									"key": "Content-Type",
									"value": "application/json; charset=utf-8",
									"name": "Content-Type",
									"description": ""
								},
								{
									"key": "Date",
									"value": "Wed, 11 Jan 2017 10:38:42 GMT",
									"name": "Date",
									"description": ""
								},
								{
									"key": "ETag",
									"value": "W/\"569-P9uLZEIyoPfMmQ+U0mTO1A\"",
									"name": "ETag",
									"description": ""
								},
								{
									"key": "Server",
									"value": "nginx/1.10.1",
									"name": "Server",
									"description": ""
								},
								{
									"key": "Vary",
									"value": "X-HTTP-Method-Override, Accept-Encoding",
									"name": "Vary",
									"description": ""
								},
								{
									"key": "set-cookie",
									"value": "sails.sid=s%3A55y5Ll7HpTzt_hKuw6N54k4N04ilmMdn.uCPCHttP5DmI%2BdBw2I9NZL55lFFOzz4XxS4qAHv47gI; Path=/; HttpOnly",
									"name": "set-cookie",
									"description": ""
								},
								{
									"key": "transfer-encoding",
									"value": "chunked",
									"name": "transfer-encoding",
									"description": ""
								}
							],
							"cookie": [
								{
									"expires": "Invalid Date",
									"httpOnly": true,
									"domain": "postman-echo.com",
									"path": "/",
									"secure": false,
									"value": "s%3A55y5Ll7HpTzt_hKuw6N54k4N04ilmMdn.uCPCHttP5DmI%2BdBw2I9NZL55lFFOzz4XxS4qAHv47gI",
									"key": "sails.sid"
								}
							],
							"body": "{\"id\":\"0c42230c-c8e4-4ca0-a4aa-d393971de8b8\",\"name\":\"Sample Postman Collection\",\"description\":\"A sample collection to demonstrate collections as a set of related requests\",\"order\":[\"3d04ed83-dc1e-40ec-923c-16aa92509e50\",\"e02f8160-fb41-4633-be80-cc7d701e85b4\",\"77bd6d4d-1060-4927-aa5c-dcdba7f750cf\"],\"folders\":[],\"requests\":[{\"id\":\"3d04ed83-dc1e-40ec-923c-16aa92509e50\",\"name\":\"A simple GET request\",\"collectionId\":\"1dd68aff-a3fa-4f52-904f-5b75053bc9d9\",\"method\":\"GET\",\"headers\":\"\",\"data\":[],\"rawModeData\":\"\",\"tests\":\"tests['response code is 200'] = (responseCode.code === 200);\",\"preRequestScript\":\"\",\"url\":\"https://postman-echo.com/get?source=newman-sample-github-collection\"},{\"id\":\"e02f8160-fb41-4633-be80-cc7d701e85b4\",\"name\":\"A simple POST request\",\"collectionId\":\"1dd68aff-a3fa-4f52-904f-5b75053bc9d9\",\"method\":\"POST\",\"headers\":\"Content-Type: text/plain\",\"dataMode\":\"raw\",\"data\":[],\"rawModeData\":\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\",\"url\":\"https://postman-echo.com/post\"},{\"id\":\"77bd6d4d-1060-4927-aa5c-dcdba7f750cf\",\"name\":\"A simple POST request with JSON body\",\"collectionId\":\"1dd68aff-a3fa-4f52-904f-5b75053bc9d9\",\"method\":\"POST\",\"headers\":\"Content-Type: application/json\",\"dataMode\":\"raw\",\"data\":[],\"rawModeData\":\"{\\\"text\\\":\\\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\\\"}\",\"url\":\"https://postman-echo.com/post\"}]}"
						}
					]
				}
			],
			"protocolProfileBehavior": {}
		},
		{
			"name": "Auth: Digest",
			"item": [
				{
					"name": "DigestAuth Request",
					"event": [
						{
							"listen": "test",
							"script": {
								"type": "text/javascript",
								"exec": [
									"tests[\"response code is 401\"] = responseCode.code === 401;",
									"tests[\"response has WWW-Authenticate header\"] = (postman.getResponseHeader('WWW-Authenticate'));",
									"",
									"var authenticateHeader = postman.getResponseHeader('WWW-Authenticate'),",
									"    realmStart = authenticateHeader.indexOf('\"',authenticateHeader.indexOf(\"realm\")) + 1 ,",
									"    realmEnd = authenticateHeader.indexOf('\"',realmStart),",
									"    realm = authenticateHeader.slice(realmStart,realmEnd),",
									"    nonceStart = authenticateHeader.indexOf('\"',authenticateHeader.indexOf(\"nonce\")) + 1,",
									"    nonceEnd = authenticateHeader.indexOf('\"',nonceStart),",
									"    nonce = authenticateHeader.slice(nonceStart,nonceEnd);",
									"    ",
									"postman.setGlobalVariable('echo_digest_realm', realm);",
									"postman.setGlobalVariable('echo_digest_nonce', nonce);"
								],
								"id": "6b069ec3-8baf-4497-8418-6f084d2bc1a0"
							}
						}
					],
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/digest-auth",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"digest-auth"
							]
						},
						"description": "Performing a simple `GET` request to this endpoint returns status code `401 Unauthorized` with `WWW-Authenticate` header containing information to successfully authenticate subsequent requests.\nThe `WWW-Authenticate` header must be processed to extract `realm` and `nonce` values to hash subsequent requests.\n\nWhen this request is executed within Postman, the script attached with this request does the hard work of extracting realm and nonce from the header and set it as [global variables](https://www.getpostman.com/docs/environments#global-variables?source=echo-collection-app-onboarding) named `echo_digest_nonce` and `echo_digest_realm`.\nThese variables are re-used in subsequent request for seamless integration of the two requests."
					},
					"response": []
				}
			],
			"description": "Digest authentication protects an endpoint with a username and password without actually transmitting the password over network.\nOne has to apply a hash function (like MD5, etc) to the username and password before sending them over the network.\n\n> Username: `postman`\n>\n> Password: `password`\n\nUnlike Basic-Auth, authentication happens using two consecutive requests where the first request returns `401 Unauthorised` along with `WWW-Authenticate` header containing information that needs to be used to authenticate subsequent calls.\n\nTo know more about digest authentication, refer to the [Digest Access Authentication](https://en.wikipedia.org/wiki/Digest_access_authentication) wikipedia article.\nThe article on [authentication helpers](https://www.getpostman.com/docs/helpers#digest-auth) elaborates how to use the same within the Postman app.",
			"protocolProfileBehavior": {}
		},
		{
			"name": "Custom",
			"item": [
				{
					"name": "Path variables",
					"request": {
						"method": "GET",
						"header": [],
						"url": {
							"raw": "https://postman-echo.com/:method/{{hello_{{var}}}}",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								":method",
								"{{hello_{{var}}}}"
							],
							"variable": [
								{
									"key": "method",
									"value": "get",
									"description": "An HTTP method."
								}
							]
						}
					},
					"response": []
				},
				{
					"name": "PUT Custom",
					"request": {
						"method": "PUT",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "{\n    \"hello\": \"there\",\n    \"my\": 1,\n    \"name\": true,\n    \"is\": {\n        \"legally\": \"{{var}}\",\n        \"num\": {{var_int}},\n        \"mixed\": [\n            {\n                \"name\": \"Kevin\"\n            },\n            true,\n            38,\n            \"Kevin\"\n        ]\n    }\n}",
							"options": {
								"raw": {
									"language": "json"
								}
							}
						},
						"url": {
							"raw": "https://postman-echo.com/put",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"put"
							]
						}
					},
					"response": []
				},
				{
					"name": "PUT Custom Copy",
					"request": {
						"method": "PUT",
						"header": [],
						"body": {
							"mode": "raw",
							"raw": "[\n    {\n        \"hello\": \"there\"\n    },\n    {\n        \"my\": 1,\n        \"name\": true,\n        \"is\": {\n            \"legally\": \"{{var}}\",\n            \"num\": {{var_int}},\n            \"mixed\": [\n                {\n                    \"name\": \"Kevin\"\n                },\n                true,\n                38,\n                \"Kevin\"\n            ]\n        }\n    },\n    {\n        \"authenticated\": true\n    }\n]",
							"options": {
								"raw": {
									"language": "json"
								}
							}
						},
						"url": {
							"raw": "https://postman-echo.com/put",
							"protocol": "https",
							"host": [
								"postman-echo",
								"com"
							],
							"path": [
								"put"
							]
						}
					},
					"response": []
				}
			],
			"protocolProfileBehavior": {}
		}
	],
	"event": [
		{
			"listen": "prerequest",
			"script": {
				"id": "14401fa1-d351-4917-9e05-02dbc3b2918a",
				"type": "text/javascript",
				"exec": [
					""
				]
			}
		},
		{
			"listen": "test",
			"script": {
				"id": "99b9a829-8941-40b4-98d9-bd96aafcf69d",
				"type": "text/javascript",
				"exec": [
					""
				]
			}
		}
	],
	"variable": [
		{
			"id": "da590c75-29f6-4f14-936b-03c10167f7f9",
			"key": "var",
			"value": "variable_value"
		},
		{
			"id": "acb7c317-5cb4-4d84-a985-7971f42b450d",
			"key": "hello_variable_value",
			"value": "hello"
		},
		{
			"id": "881ec263-5887-4fcc-bd54-efaf019d3eb7",
			"key": "var_int",
			"value": "1000"
		}
	],
	"protocolProfileBehavior": {}
}"#;
    static OPENAPI: &'static str = r#"---
openapi: 3.0.3
info:
  title: Postman Echo
  description: "Postman Echo is service you can use to test your REST clients and make sample API calls. It provides endpoints for `GET`, `POST`, `PUT`, various auth mechanisms and other utility endpoints.\n\nThe documentation for the endpoints as well as example responses can be found at [https://postman-echo.com](https://postman-echo.com/?source=echo-collection-app-onboarding)"
  version: 1.0.0
  contact: {}
servers:
  - url: "https://postman-echo.com"
paths:
  /auth/hawk:
    get:
      tags:
        - Authentication Methods
      summary: Hawk Auth
      description: "This endpoint is a Hawk Authentication protected endpoint. [Hawk authentication](https://github.com/hueniverse/hawk) is a widely used protocol for protecting API endpoints. One of Hawk's main goals is to enable HTTP authentication for services that do not use TLS (although it can be used in conjunction with TLS as well).\n\nIn order to use this endpoint, select the \"Hawk Auth\" helper inside Postman, and set the following values:\n\nHawk Auth ID: `dh37fgj492je`\n\nHawk Auth Key: `werxhqb98rpaxn39848xrunpaw3489ruxnpa98w4rxn`\n\nAlgorithm: `sha256`\n\nThe rest of the values are optional, and can be left blank. Hitting send should give you a response with a status code of 200 OK."
      operationId: hawkAuth
      responses:
        "200":
          description: Success
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Date:
              schema:
                type: string
                example: "Thu, 31 Mar 2016 11:12:16 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Server-Authorization:
              schema:
                type: string
                example: "Hawk mac=\"vRrUzDdcHu2NaNts/r4zg2xmXMdX8wPiTGTM398BDRg=\", hash=\"qmtflETMybaZiOQ2dLT17yiRunFT5OCIxZRZ0boQaiE=\""
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
            transfer-encoding:
              schema:
                type: string
                example: chunked
          content:
            application/json:
              schema:
                type: object
                properties:
                  message:
                    type: string
                    example: Hawk Authentication successful
                  status:
                    type: string
                    example: pass
              examples:
                Success:
                  value:
                    message: Hawk Authentication successful
                    status: pass
  /basic-auth:
    get:
      tags:
        - Authentication Methods
      summary: Basic Auth
      description: "This endpoint simulates a **basic-auth** protected endpoint. \nThe endpoint accepts a default username and password and returns a status code of `200 ok` only if the same is provided. \nOtherwise it will return a status code `401 unauthorized`.\n\n> Username: `postman`\n> \n> Password: `password`\n\nTo use this endpoint, send a request with the header `Authorization: Basic cG9zdG1hbjpwYXNzd29yZA==`. \nThe cryptic latter half of the header value is a base64 encoded concatenation of the default username and password. \nUsing Postman, to send this request, you can simply fill in the username and password in the \"Authorization\" tab and Postman will do the rest for you.\n\nTo know more about basic authentication, refer to the [Basic Access Authentication](https://en.wikipedia.org/wiki/Basic_access_authentication) wikipedia article.\nThe article on [authentication helpers](https://www.getpostman.com/docs/helpers#basic-auth?source=echo-collection-app-onboarding) elaborates how to use the same within the Postman app."
      operationId: basicAuth
      responses:
        "200":
          description: "200"
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "42"
            Date:
              schema:
                type: string
                example: "Sat, 31 Oct 2015 06:38:25 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  authenticated:
                    type: boolean
                    example: true
              examples:
                "200":
                  value:
                    authenticated: true
  /cookies:
    get:
      tags:
        - Cookie Manipulation
      summary: Get Cookies
      description: "Use this endpoint to get a list of all cookies that are stored with respect to this domain. Whatever key-value pairs that has been previously set by calling the \"Set Cookies\" endpoint, will be returned as response JSON."
      operationId: getCookies
      responses:
        "200":
          description: Cookies
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "46"
            Date:
              schema:
                type: string
                example: "Thu, 29 Oct 2015 06:16:29 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  cookies:
                    type: object
                    properties:
                      foo2:
                        type: string
                        example: bar
              examples:
                Cookies:
                  value:
                    cookies:
                      foo2: bar
  /cookies/delete:
    get:
      tags:
        - Cookie Manipulation
      summary: Delete Cookies
      description: One or more cookies that has been set for this domain can be deleted by providing the cookie names as part of the URL parameter. The response of this request is a JSON containing the list of currently set cookies.
      operationId: deleteCookies
      parameters:
        - name: foo1
          in: query
          schema:
            type: string
        - name: foo2
          in: query
          schema:
            type: string
      responses:
        "200":
          description: Cookies Response
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "46"
            Date:
              schema:
                type: string
                example: "Thu, 29 Oct 2015 06:16:00 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  cookies:
                    type: object
                    properties:
                      foo2:
                        type: string
                        example: bar
              examples:
                Cookies Response:
                  value:
                    cookies:
                      foo2: bar
  /cookies/set:
    get:
      tags:
        - Cookie Manipulation
      summary: Set Cookies
      description: "The cookie setter endpoint accepts a list of cookies and their values as part of URL parameters of a `GET` request. These cookies are saved and can be subsequently retrieved or deleted. The response of this request returns a JSON with all cookies listed.\n\nTo set your own set of cookies, simply replace the URL parameters \"foo1=bar1&foo2=bar2\" with your own set of key-value pairs."
      operationId: setCookies
      parameters:
        - name: foo1
          in: query
          schema:
            type: string
            example: bar1
        - name: foo2
          in: query
          schema:
            type: string
            example: bar2
      responses:
        "200":
          description: Cookies
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "51"
            Date:
              schema:
                type: string
                example: "Thu, 29 Oct 2015 06:15:28 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  cookies:
                    type: object
                    properties:
                      foo1:
                        type: string
                        example: bar
                      foo2:
                        type: string
                        example: bar
              examples:
                Cookies:
                  value:
                    cookies:
                      foo1: bar
                      foo2: bar
  /deflate:
    get:
      tags:
        - Utilities
      summary: Deflate Compressed Response
      description: "This endpoint returns the response using [deflate compression algoritm](https://en.wikipedia.org/wiki/DEFLATE). \nThe uncompressed response is a JSON string containing the details of the request sent by the client. For this endpoint to work, one should request with `Accept-encoding` header containing `deflate` as part of its value. Postman supports gzip, deflate and SDCH decoding and automatically sends them as part of the request.\n\nHTTP Compression allows the server to send responses in a compressed format, which is uncompressed by the client before processing. This reduces network bandwidth consumption at the cost of increase in CPU usage.\nTo know more about this, refer the [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression) wikipedia article."
      operationId: deflateCompressedResponse
      responses:
        "200":
          description: ""
  /delay/2:
    get:
      tags:
        - Utilities
      summary: Delay Response
      description: "Using this endpoint one can configure how long it takes for the server to come back with a response. Appending a number to the URL defines the time (in seconds) the server will wait before responding.\n\nNote that a maximum delay of 10 seconds is accepted by the server."
      operationId: delayResponse
      responses:
        "200":
          description: success-response
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Length:
              schema:
                type: string
                example: "13"
            Date:
              schema:
                type: string
                example: "Mon, 02 Jan 2017 09:19:03 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"d-t/L/D5c0SDl+MoXtKdSVOg\""
            Server:
              schema:
                type: string
                example: nginx/1.10.1
            Vary:
              schema:
                type: string
                example: Accept-Encoding
          content:
            application/json:
              schema:
                type: object
                properties:
                  delay:
                    type: string
                    example: "3"
              examples:
                success-response:
                  value:
                    delay: "3"
  /delete:
    delete:
      tags:
        - Request Methods
      summary: DELETE Request
      description: "The HTTP `DELETE` method is used to delete resources on a server. The exact\nuse of `DELETE` requests depends on the server implementation. In general, \n`DELETE` requests support both, Query String parameters as well as a Request \nBody.\n\nThis endpoint accepts an HTTP `DELETE` request and provides debug information\nsuch as the HTTP headers, Query String arguments, and the Request Body."
      operationId: deleteRequest
      requestBody:
        content:
          text/plain:
            example: This is expected to be sent back as part of response body.
      responses:
        "200":
          description: ""
  /digest-auth:
    get:
      tags:
        - "Auth: Digest"
      summary: DigestAuth Request
      description: "Performing a simple `GET` request to this endpoint returns status code `401 Unauthorized` with `WWW-Authenticate` header containing information to successfully authenticate subsequent requests.\nThe `WWW-Authenticate` header must be processed to extract `realm` and `nonce` values to hash subsequent requests.\n\nWhen this request is executed within Postman, the script attached with this request does the hard work of extracting realm and nonce from the header and set it as [global variables](https://www.getpostman.com/docs/environments#global-variables?source=echo-collection-app-onboarding) named `echo_digest_nonce` and `echo_digest_realm`.\nThese variables are re-used in subsequent request for seamless integration of the two requests."
      operationId: digestAuthRequest
      responses:
        "200":
          description: ""
  /encoding/utf8:
    get:
      tags:
        - Utilities
      summary: Get UTF8 Encoded Response
      description: "If a response of an endpoint requires to send data beyond the basic English / ASCII character set, the `charset` parameter in the `Content-Type` response header defines the character encoding policy.\n\nThis endpoint returns an `UTF8` character encoded response body with text in various languages such as Greek, Latin, East Asian, etc. Postman can interpret the character encoding and use appropriate methods to display the character set in responses."
      operationId: getUtf8EncodedResponse
      responses:
        "200":
          description: ""
  /get:
    get:
      tags:
        - Request Methods
      summary: GET Request
      description: "The HTTP `GET` request method is meant to retrieve data from a server. The data\nis identified by a unique URI (Uniform Resource Identifier). \n\nA `GET` request can pass parameters to the server using \"Query String \nParameters\". For example, in the following request,\n\n> http://example.com/hi/there?hand=wave\n\nThe parameter \"hand\" has the value \"wave\".\n\nThis endpoint echoes the HTTP headers, request parameters and the complete\nURI requested."
      operationId: getRequest
      parameters:
        - name: foo1
          in: query
          schema:
            type: string
            example: bar1
        - name: foo2
          in: query
          schema:
            type: string
            example: bar2
      responses:
        "200":
          description: GET Request Woops
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "249"
            Date:
              schema:
                type: string
                example: "Tue, 11 Jun 2019 10:43:13 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"161-aLhNcsGArlgLSKbxPqfBW3viHPI\""
            Server:
              schema:
                type: string
                example: nginx
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            set-cookie:
              schema:
                type: string
                example: sails.sid=s%3AGz-wblZgXE8FCDq7aJpx_tUgZUcG3Nsw.LdNEN8L0C7nGWkvGLwvdw6R2s6Syjr%2FzkvyevA8qR0c; Path=/; HttpOnly
          content:
            application/json:
              schema:
                type: object
                properties:
                  args:
                    type: object
                    properties:
                      foo1:
                        type: string
                        example: bar1
                      foo2:
                        type: string
                        example: bar2
                  headers:
                    type: object
                    properties:
                      accept:
                        type: string
                        example: "*/*"
                      accept-encoding:
                        type: string
                        example: "gzip, deflate"
                      cache-control:
                        type: string
                        example: no-cache
                      host:
                        type: string
                        example: postman-echo.com
                      postman-token:
                        type: string
                        example: 5c27cd7d-6b16-4e5a-a0ef-191c9a3a275f
                      user-agent:
                        type: string
                        example: PostmanRuntime/7.6.1
                      x-forwarded-port:
                        type: string
                        example: "443"
                      x-forwarded-proto:
                        type: string
                        example: https
                  url:
                    type: string
                    example: "https://postman-echo.com/get?foo1=bar1&foo2=bar2"
              examples:
                GET Request Woops:
                  value:
                    args:
                      foo1: bar1
                      foo2: bar2
                    headers:
                      accept: "*/*"
                      accept-encoding: "gzip, deflate"
                      cache-control: no-cache
                      host: postman-echo.com
                      postman-token: 5c27cd7d-6b16-4e5a-a0ef-191c9a3a275f
                      user-agent: PostmanRuntime/7.6.1
                      x-forwarded-port: "443"
                      x-forwarded-proto: https
                    url: "https://postman-echo.com/get?foo1=bar1&foo2=bar2"
  /gzip:
    get:
      tags:
        - Utilities
      summary: GZip Compressed Response
      description: "This endpoint returns the response using [gzip compression algoritm](https://en.wikipedia.org/wiki/Gzip).\nThe uncompressed response is a JSON string containing the details of the request sent by the client. For this endpoint to work, one should request with `Accept-encoding` header containing `gzip` as part of its value. Postman supports gzip, deflate and SDCH decoding and automatically sends them as part of the request.\n\nHTTP Compression allows the server to send responses in a compressed format, which is uncompressed by the client before processing. This reduces network bandwidth consumption at the cost of increase in CPU usage.\nTo know more about this, refer the [HTTP Compression](https://en.wikipedia.org/wiki/HTTP_compression) wikipedia article."
      operationId: gZipCompressedResponse
      responses:
        "200":
          description: ""
  /headers:
    get:
      tags:
        - Headers
      summary: Request Headers
      description: "A `GET` request to this endpoint returns the list of all request headers as part of the response JSON.\nIn Postman, sending your own set of headers through the [Headers tab](https://www.getpostman.com/docs/requests#headers?source=echo-collection-app-onboarding) will reveal the headers as part of the response."
      operationId: requestHeaders
      responses:
        "200":
          description: my-sample-header
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "342"
            Date:
              schema:
                type: string
                example: "Thu, 31 Mar 2016 11:14:00 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  headers:
                    type: object
                    properties:
                      accept:
                        type: string
                        example: "*/*"
                      accept-encoding:
                        type: string
                        example: "gzip, deflate, sdch"
                      accept-language:
                        type: string
                        example: "en-US,en;q=0.8"
                      cache-control:
                        type: string
                        example: no-cache
                      host:
                        type: string
                        example: echo.getpostman.com
                      my-sample-header:
                        type: string
                        example: Lorem ipsum dolor sit amet
                      postman-token:
                        type: string
                        example: 3c8ea80b-f599-fba6-e0b4-a0910440e7b6
                      user-agent:
                        type: string
                        example: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.110 Safari/537.36"
                      x-forwarded-port:
                        type: string
                        example: "443"
                      x-forwarded-proto:
                        type: string
                        example: https
              examples:
                my-sample-header:
                  value:
                    headers:
                      accept: "*/*"
                      accept-encoding: "gzip, deflate, sdch"
                      accept-language: "en-US,en;q=0.8"
                      cache-control: no-cache
                      host: echo.getpostman.com
                      my-sample-header: Lorem ipsum dolor sit amet
                      postman-token: 3c8ea80b-f599-fba6-e0b4-a0910440e7b6
                      user-agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.110 Safari/537.36"
                      x-forwarded-port: "443"
                      x-forwarded-proto: https
  /ip:
    get:
      tags:
        - Utilities
      summary: IP address in JSON format
      description: "A simple `GET` request to return the IP address of the source request in the following `JSON` format:\n\n```json\n{\n  ip: \"request-ip-address\"\n}\n```"
      operationId: ipAddressInJsonFormat
      responses:
        "200":
          description: ""
  /oauth1:
    get:
      tags:
        - Authentication Methods
      summary: OAuth1.0 (verify signature)
      description: "OAuth1.0a is a specification that defines a protocol that can be used by one\nservice to access \"protected\" resources (endpoints) on another service. A\nmajor part of OAuth1.0 is HTTP Request Signing. This endpoint allows you to \ncheck whether the request calculation works properly in the client. \n\nThe endpoint supports the HTTP ``Authorization`` header. In case the signature\nverification fails, the endpoint provides the four debug values,\n\n* ``base_uri``\n* ``normalized_param_string``\n* ``base_string``\n* ``signing_key``\n\nFor more details about these parameters, check the [OAuth1.0a Specification](http://oauth.net/core/1.0a/)\n\nIn order to use this endpoint, you can set the following values:\n\n> Consumer Key: ``RKCGzna7bv9YD57c``\n>\n> Consumer Secret: ``D+EdQ-gs$-%@2Nu7``\n\nIf you are using Postman, also check the \"Add params to header\" and \n\"Auto add parameters\" boxes."
      operationId: oAuth1.0(verifySignature)
      responses:
        "200":
          description: "200"
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "95"
            Date:
              schema:
                type: string
                example: "Thu, 25 Aug 2016 10:34:23 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"4e-Cq3UhvpVSyl6R6204lPVIA\""
            Server:
              schema:
                type: string
                example: nginx/1.8.1
            Vary:
              schema:
                type: string
                example: Accept-Encoding
          content:
            application/json:
              schema:
                type: object
                properties:
                  message:
                    type: string
                    example: OAuth-1.0a signature verification was successful
                  status:
                    type: string
                    example: pass
              examples:
                "200":
                  value:
                    message: OAuth-1.0a signature verification was successful
                    status: pass
        "401":
          description: "401"
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Length:
              schema:
                type: string
                example: "536"
            Date:
              schema:
                type: string
                example: "Thu, 25 Aug 2016 10:34:55 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"218-SGnurnTsu5qV5cCYWxsJlg\""
            Server:
              schema:
                type: string
                example: nginx/1.8.1
            Vary:
              schema:
                type: string
                example: Accept-Encoding
          content:
            application/json:
              schema:
                type: object
                properties:
                  base_string:
                    type: string
                    example: GET&https%3A%2F%2Fecho.getpostman.com%2Foauth1&oauth_consumer_key%3DRKCGzna7bv9YD57c_wrong%26oauth_nonce%3D8LTsU2%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1472121295%26oauth_version%3D1.0
                  base_uri:
                    type: string
                    example: "https://echo.getpostman.com/oauth1"
                  message:
                    type: string
                    example: HMAC-SHA1 verification failed
                  normalized_param_string:
                    type: string
                    example: oauth_consumer_key=RKCGzna7bv9YD57c_wrong&oauth_nonce=8LTsU2&oauth_signature_method=HMAC-SHA1&oauth_timestamp=1472121295&oauth_version=1.0
                  signing_key:
                    type: string
                    example: D%2BEdQ-gs%24-%25%402Nu7&
                  status:
                    type: string
                    example: fail
              examples:
                "401":
                  value:
                    base_string: GET&https%3A%2F%2Fecho.getpostman.com%2Foauth1&oauth_consumer_key%3DRKCGzna7bv9YD57c_wrong%26oauth_nonce%3D8LTsU2%26oauth_signature_method%3DHMAC-SHA1%26oauth_timestamp%3D1472121295%26oauth_version%3D1.0
                    base_uri: "https://echo.getpostman.com/oauth1"
                    message: HMAC-SHA1 verification failed
                    normalized_param_string: oauth_consumer_key=RKCGzna7bv9YD57c_wrong&oauth_nonce=8LTsU2&oauth_signature_method=HMAC-SHA1&oauth_timestamp=1472121295&oauth_version=1.0
                    signing_key: D%2BEdQ-gs%24-%25%402Nu7&
                    status: fail
  /patch:
    patch:
      tags:
        - Request Methods
      summary: PATCH Request
      description: "The HTTP `PATCH` method is used to update resources on a server. The exact\nuse of `PATCH` requests depends on the server in question. There are a number\nof server implementations which handle `PATCH` differently. Technically, \n`PATCH` supports both Query String parameters and a Request Body.\n\nThis endpoint accepts an HTTP `PATCH` request and provides debug information\nsuch as the HTTP headers, Query String arguments, and the Request Body."
      operationId: patchRequest
      requestBody:
        content:
          text/plain:
            example: This is expected to be sent back as part of response body.
      responses:
        "200":
          description: ""
  /post:
    post:
      tags:
        - Request Methods
      summary: POST Form Data
      description: "The HTTP `POST` request method is meant to transfer data to a server \n(and elicit a response). What data is returned depends on the implementation\nof the server.\n\nA `POST` request can pass parameters to the server using \"Query String \nParameters\", as well as the Request Body. For example, in the following request,\n\n> POST /hi/there?hand=wave\n>\n> <request-body>\n\nThe parameter \"hand\" has the value \"wave\". The request body can be in multiple\nformats. These formats are defined by the MIME type of the request. The MIME \nType can be set using the ``Content-Type`` HTTP header. The most commonly used \nMIME types are:\n\n* `multipart/form-data`\n* `application/x-www-form-urlencoded`\n* `application/json`\n\nThis endpoint echoes the HTTP headers, request parameters, the contents of\nthe request body and the complete URI requested when data is sent as a form parameter."
      operationId: postFormData
      requestBody:
        content:
          application/form-urlencoded:
            schema:
              type: object
              properties:
                foo1:
                  type: string
                  example: bar1
                foo2:
                  type: string
                  example: bar2
            example:
              foo1: bar1
              foo2: bar2
      responses:
        "200":
          description: ""
  /put:
    put:
      tags:
        - Custom
      summary: PUT Custom Copy
      description: PUT Custom Copy
      operationId: putCustomCopy
      requestBody:
        content:
          application/json:
            schema:
              type: array
              items:
                type: object
                properties:
                  authenticated:
                    type: boolean
                    example: true
                  hello:
                    type: string
                    example: there
                  is:
                    type: object
                    properties:
                      legally:
                        type: string
                        example: variable_value
                      mixed:
                        type: array
                        items:
                          anyOf:
                            - type: object
                              properties:
                                name:
                                  type: string
                                  example: Kevin
                            - type: boolean
                              example: true
                            - type: number
                              example: 38
                            - type: string
                              example: Kevin
                        example:
                          - name: Kevin
                          - true
                          - 38
                          - Kevin
                      num:
                        type: number
                        example: 1000
                  my:
                    type: number
                    example: 1
                  name:
                    type: boolean
                    example: true
              example:
                - hello: there
                - is:
                    legally: variable_value
                    mixed:
                      - name: Kevin
                      - true
                      - 38
                      - Kevin
                    num: 1000
                  my: 1
                  name: true
                - authenticated: true
            example:
              - hello: there
              - is:
                  legally: variable_value
                  mixed:
                    - name: Kevin
                    - true
                    - 38
                    - Kevin
                  num: 1000
                my: 1
                name: true
              - authenticated: true
      responses:
        "200":
          description: ""
  /response-headers:
    get:
      tags:
        - Headers
      summary: Response Headers
      description: "This endpoint causes the server to send custom set of response headers. Providing header values as part of the URL parameters of a `GET` request to this endpoint returns the same as part of response header.\n\nTo send your own set of headers, simply add or replace the the URL parameters with your own set."
      operationId: responseHeaders
      parameters:
        - name: foo1
          in: query
          schema:
            type: string
            example: bar1
        - name: foo2
          in: query
          schema:
            type: string
            example: bar2
      responses:
        "200":
          description: Response headers
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "71"
            Date:
              schema:
                type: string
                example: "Thu, 31 Mar 2016 11:14:18 GMT"
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
            test:
              schema:
                type: string
                example: response_headers
          content:
            application/json:
              schema:
                type: object
                properties:
                  Content-Type:
                    type: string
                    example: text/html
                  test:
                    type: string
                    example: response_headers
              examples:
                Response headers:
                  value:
                    Content-Type: text/html
                    test: response_headers
  /status/200:
    get:
      tags:
        - Utilities
      summary: Response Status Code
      description: "This endpoint allows one to instruct the server which status code to respond with.\n\nEvery response is accompanied by a status code. The status code provides a summary of the nature of response sent by the server. For example, a status code of `200` means everything is okay with the response and a code of `404` implies that the requested URL does not exist on server. \nA list of all valid HTTP status code can be found at the [List of Status Codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes) wikipedia article. When using Postman, the response status code is described for easy reference.\n\nNote that if an invalid status code is requested to be sent, the server returns a status code of `400 Bad Request`."
      operationId: responseStatusCode
      responses:
        "200":
          description: "200"
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Length:
              schema:
                type: string
                example: "14"
            Date:
              schema:
                type: string
                example: "Thu, 31 Mar 2016 11:58:47 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"e-1056260003\""
            Server:
              schema:
                type: string
                example: nginx/1.6.2
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            X-Powered-By:
              schema:
                type: string
                example: Sails <sailsjs.org>
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: number
                    example: 200
              examples:
                "200":
                  value:
                    status: 200
  /stream/5:
    get:
      tags:
        - Utilities
      summary: Streamed Response
      description: "This endpoint allows one to recieve streaming http response using [chunked transfer encoding](https://en.wikipedia.org/wiki/Chunked_transfer_encoding) of a configurable length.\n\nA streaming response does not wait for the entire response to be generated on server before flushing it out. This implies that for a fairly large response, parts of it can be streamed to the requestee as and when it is generated on server. The client can then take actions of processing this partially received data."
      operationId: streamedResponse
      responses:
        "200":
          description: ""
  /time/add:
    get:
      tags:
        - Utilities / Date and Time
      summary: Time addition
      description: "A simple `GET` request to `/time/add` to add units of time to the specified / current timestamp (as provided in the `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds` query parameters).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `sum` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  sum: \"sum of (provided / current) and provided timestamps\"\n}\n```"
      operationId: timeAddition
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: years
          in: query
          schema:
            type: string
            example: "100"
      responses:
        "200":
          description: ""
  /time/after:
    get:
      tags:
        - Utilities / Date and Time
      summary: After comparisons
      description: "A simple `GET` request to `/time/after` to check if the provided timestamps is after a comparison `target` (query parameter).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `after` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  after: true/false\n}\n```"
      operationId: afterComparisons
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: target
          in: query
          schema:
            type: string
            example: 2017-10-10
      responses:
        "200":
          description: ""
  /time/before:
    get:
      tags:
        - Utilities / Date and Time
      summary: Before comparisons
      description: "A simple `GET` request to `/time/before` to check if the provided timestamps is before a comparison `target` (query parameter).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `before` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  before: true/false\n}\n```"
      operationId: beforeComparisons
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: target
          in: query
          schema:
            type: string
            example: 2017-10-10
      responses:
        "200":
          description: ""
  /time/between:
    get:
      tags:
        - Utilities / Date and Time
      summary: Between timestamps
      description: "A simple `GET` request to `/time/between` to check if the provided timestamp is between a range specified by the `start` and `end` query parameters. A resolution limit can also be specified by the `unit` query parameter.\n\nFor instance, for a resolution `unit` of `month`,\n`2016-10-05` does lie between `2016-11-02` and `2016-09-01`.\n\nThis endpoint also accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `between` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  between: true/false\n}\n```"
      operationId: betweenTimestamps
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: start
          in: query
          schema:
            type: string
            example: 2017-10-10
        - name: end
          in: query
          schema:
            type: string
            example: 2019-10-10
      responses:
        "200":
          description: ""
  /time/format:
    get:
      tags:
        - Utilities / Date and Time
      summary: Format timestamp
      description: "A simple `GET` request to `/time/format` to convert the timestamp to any desired valid format.\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `format` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  format: \"formatted-timestamp\"\n}\n```"
      operationId: formatTimestamp
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: format
          in: query
          schema:
            type: string
            example: mm
      responses:
        "200":
          description: ""
  /time/leap:
    get:
      tags:
        - Utilities / Date and Time
      summary: Leap year check
      description: "A simple `GET` request to `/time/leap` to check if the provided/current timestamp belongs to a leap year.\n\nThis endpoint also accepts `timestamp`, `locale`, `format`, `strict`, and `target` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `leap` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  leap: true/false\n}\n```"
      operationId: leapYearCheck
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
      responses:
        "200":
          description: ""
  /time/now:
    get:
      tags:
        - Utilities / Date and Time
      summary: Current UTC time
      description: "A simple `GET` request to `/time/now` to return the current timestamp as a UTC string.\n\n```\nFri, 04 Nov 2016 09:00:46 GMT\n```"
      operationId: currentUtcTime
      responses:
        "200":
          description: time as text
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Content-Length:
              schema:
                type: string
                example: "49"
            Date:
              schema:
                type: string
                example: "Wed, 11 Jan 2017 10:27:12 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"1d-2jJhkzratfVX9VZ0+raHbw\""
            Server:
              schema:
                type: string
                example: nginx/1.10.1
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            set-cookie:
              schema:
                type: string
                example: sails.sid=s%3A2lT3TO7qS1tadeSAp4axl-NcXG9CV6Rf.HGqLY%2FlKEKY4fgCLePaAZs3tCHp%2Bglf7ZOJYlonGeig; Path=/; HttpOnly
          content:
            text/plain:
              examples:
                time as text:
                  value: "Wed, 11 Jan 2017 10:27:12 GMT"
  /time/object:
    get:
      tags:
        - Utilities / Date and Time
      summary: Object representation
      description: "A simple `GET` request to `/time/object` to return the current / provided timestamp as a JSON object.\n\nFor instance, if the `unit` has been specified as `month`, the returned timestamp would indicate the beginning of the current month. Similar results are returned for other units of time, like: `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds`\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  years: 2016,\n  months: 10,\n  days: 10,\n  hours: 23,\n  minutes: 34,\n  seconds: 20,\n  milliseconds: 980\n}\n```"
      operationId: objectRepresentation
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
      responses:
        "200":
          description: ""
  /time/start:
    get:
      tags:
        - Utilities / Date and Time
      summary: Start of time
      description: "A simple `GET` request to `/time/start` to return a relative timstamp in the past from the specified / current timestamp (as provided in the `unit` query parameter).\n\nFor instance, if the `unit` has been specified as `month`, the returned timestamp would indicate the beginning of the current month. Similar results are returned for other units of time, like: `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds`\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `start` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  start: \"A timestamp from the past, depending on the `unit` specified\"\n}\n```"
      operationId: startOfTime
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: unit
          in: query
          schema:
            type: string
            example: month
      responses:
        "200":
          description: ""
  /time/subtract:
    get:
      tags:
        - Utilities / Date and Time
      summary: Time subtraction
      description: "A simple `GET` request to `/time/subtract` to subtract units of time from the specified / current timestamp (as provided in the `years`, `months`, `days`, `hours`, `minutes`, `seconds`, and `milliseconds` query parameters).\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `difference` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  difference: \"difference between (provided / current) and provided timestamps\"\n}\n```"
      operationId: timeSubtraction
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: years
          in: query
          schema:
            type: string
            example: "50"
      responses:
        "200":
          description: ""
  /time/unit:
    get:
      tags:
        - Utilities / Date and Time
      summary: Extract timestamp unit
      description: "A simple `GET` request to `/time/unit` to extract the specified timestamp unit (as provided in the `unit` query parameter). The default unit returned is the `year`.\n\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a `unit` key to indicate the result. The response code is `200` for valid query parameters, and `400` otherwise.\n\n```\n{\n  unit: \"extracted-timestamp-unit\"\n}\n```"
      operationId: extractTimestampUnit
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
        - name: unit
          in: query
          schema:
            type: string
            example: day
      responses:
        "200":
          description: ""
  /time/valid:
    get:
      tags:
        - Utilities / Date and Time
      summary: Timestamp validity
      description: "A simple `GET` request to `/time/valid` to determine the validity of the timestamp, (current by default).\nThis endpoint accepts `timestamp`, `locale`, `format`, and `strict` query parameters to construct the date time instance to check against.\n\nResponses are provided in JSON format, with a valid key to indicate the result. The response code is `200`.\n\n```\n{\n  valid: true/false\n}\n```"
      operationId: timestampValidity
      parameters:
        - name: timestamp
          in: query
          schema:
            type: string
            example: 2016-10-10
      responses:
        "200":
          description: Invalid Timestamp
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Length:
              schema:
                type: string
                example: "15"
            Date:
              schema:
                type: string
                example: "Wed, 11 Jan 2017 10:27:53 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"f-/i9mO/upK91ZtL0BkKFGtw\""
            Server:
              schema:
                type: string
                example: nginx/1.10.1
            Vary:
              schema:
                type: string
                example: Accept-Encoding
            set-cookie:
              schema:
                type: string
                example: sails.sid=s%3ATNJaNxi2QCv4RPBb64sIZxQGN1h6IP3g.9sQVAijlsLsh0r7LgffxXa9k2we6UumPEVv%2Bsk4woLI; Path=/; HttpOnly
          content:
            application/json:
              schema:
                type: object
                properties:
                  valid:
                    type: boolean
                    example: false
              examples:
                Invalid Timestamp:
                  value:
                    valid: false
  /transform/collection:
    post:
      tags:
        - Utilities / Postman Collection
      summary: Transform collection from format v2 to v1
      description: Transform collection from format v2 to v1
      operationId: transformCollectionFromFormatV2ToV1
      parameters:
        - name: from
          in: query
          schema:
            type: string
            example: "2"
        - name: to
          in: query
          schema:
            type: string
            example: "1"
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                info:
                  type: object
                  properties:
                    description:
                      type: string
                      example: A sample collection to demonstrate collections as a set of related requests
                    name:
                      type: string
                      example: Sample Postman Collection
                    schema:
                      type: string
                      example: "https://schema.getpostman.com/json/collection/v2.0.0/collection.json"
                item:
                  type: array
                  items:
                    type: object
                    properties:
                      name:
                        type: string
                        example: A simple GET request
                      request:
                        type: object
                        properties:
                          body:
                            type: object
                            properties:
                              mode:
                                type: string
                                example: raw
                              raw:
                                type: string
                                example: Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...
                          header:
                            type: array
                            items:
                              type: object
                              properties:
                                key:
                                  type: string
                                  example: Content-Type
                                value:
                                  type: string
                                  example: text/plain
                            example:
                              - key: Content-Type
                                value: text/plain
                          method:
                            type: string
                            example: GET
                          url:
                            type: string
                            example: "https://postman-echo.com/get?source=newman-sample-github-collection"
                  example:
                    - name: A simple GET request
                      request:
                        method: GET
                        url: "https://postman-echo.com/get?source=newman-sample-github-collection"
                    - name: A simple POST request
                      request:
                        body:
                          mode: raw
                          raw: Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...
                        header:
                          - key: Content-Type
                            value: text/plain
                        method: POST
                        url: "https://postman-echo.com/post"
            example:
              info:
                description: A sample collection to demonstrate collections as a set of related requests
                name: Sample Postman Collection
                schema: "https://schema.getpostman.com/json/collection/v2.0.0/collection.json"
              item:
                - name: A simple GET request
                  request:
                    method: GET
                    url: "https://postman-echo.com/get?source=newman-sample-github-collection"
                - name: A simple POST request
                  request:
                    body:
                      mode: raw
                      raw: Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...
                    header:
                      - key: Content-Type
                        value: text/plain
                    method: POST
                    url: "https://postman-echo.com/post"
      responses:
        "200":
          description: Sample v1 Response
          headers:
            Connection:
              schema:
                type: string
                example: keep-alive
            Content-Encoding:
              schema:
                type: string
                example: gzip
            Date:
              schema:
                type: string
                example: "Wed, 11 Jan 2017 10:38:42 GMT"
            ETag:
              schema:
                type: string
                example: "W/\"569-P9uLZEIyoPfMmQ+U0mTO1A\""
            Server:
              schema:
                type: string
                example: nginx/1.10.1
            Vary:
              schema:
                type: string
                example: "X-HTTP-Method-Override, Accept-Encoding"
            set-cookie:
              schema:
                type: string
                example: sails.sid=s%3A55y5Ll7HpTzt_hKuw6N54k4N04ilmMdn.uCPCHttP5DmI%2BdBw2I9NZL55lFFOzz4XxS4qAHv47gI; Path=/; HttpOnly
            transfer-encoding:
              schema:
                type: string
                example: chunked
          content:
            application/json:
              schema:
                type: object
                properties:
                  description:
                    type: string
                    example: A sample collection to demonstrate collections as a set of related requests
                  folders:
                    type: array
                    items: {}
                    example: []
                  id:
                    type: string
                    example: 0c42230c-c8e4-4ca0-a4aa-d393971de8b8
                  name:
                    type: string
                    example: Sample Postman Collection
                  order:
                    type: array
                    items:
                      type: string
                      example: 3d04ed83-dc1e-40ec-923c-16aa92509e50
                    example:
                      - 3d04ed83-dc1e-40ec-923c-16aa92509e50
                      - e02f8160-fb41-4633-be80-cc7d701e85b4
                      - 77bd6d4d-1060-4927-aa5c-dcdba7f750cf
                  requests:
                    type: array
                    items:
                      type: object
                      properties:
                        collectionId:
                          type: string
                          example: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data:
                          type: array
                          items: {}
                          example: []
                        dataMode:
                          type: string
                          example: raw
                        headers:
                          type: string
                          example: ""
                        id:
                          type: string
                          example: 3d04ed83-dc1e-40ec-923c-16aa92509e50
                        method:
                          type: string
                          example: GET
                        name:
                          type: string
                          example: A simple GET request
                        preRequestScript:
                          type: string
                          example: ""
                        rawModeData:
                          type: string
                          example: ""
                        tests:
                          type: string
                          example: "tests['response code is 200'] = (responseCode.code === 200);"
                        url:
                          type: string
                          example: "https://postman-echo.com/get?source=newman-sample-github-collection"
                    example:
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        headers: ""
                        id: 3d04ed83-dc1e-40ec-923c-16aa92509e50
                        method: GET
                        name: A simple GET request
                        preRequestScript: ""
                        rawModeData: ""
                        tests: "tests['response code is 200'] = (responseCode.code === 200);"
                        url: "https://postman-echo.com/get?source=newman-sample-github-collection"
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        dataMode: raw
                        headers: "Content-Type: text/plain"
                        id: e02f8160-fb41-4633-be80-cc7d701e85b4
                        method: POST
                        name: A simple POST request
                        rawModeData: Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...
                        url: "https://postman-echo.com/post"
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        dataMode: raw
                        headers: "Content-Type: application/json"
                        id: 77bd6d4d-1060-4927-aa5c-dcdba7f750cf
                        method: POST
                        name: A simple POST request with JSON body
                        rawModeData: "{\"text\":\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\"}"
                        url: "https://postman-echo.com/post"
              examples:
                Sample v1 Response:
                  value:
                    description: A sample collection to demonstrate collections as a set of related requests
                    folders: []
                    id: 0c42230c-c8e4-4ca0-a4aa-d393971de8b8
                    name: Sample Postman Collection
                    order:
                      - 3d04ed83-dc1e-40ec-923c-16aa92509e50
                      - e02f8160-fb41-4633-be80-cc7d701e85b4
                      - 77bd6d4d-1060-4927-aa5c-dcdba7f750cf
                    requests:
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        headers: ""
                        id: 3d04ed83-dc1e-40ec-923c-16aa92509e50
                        method: GET
                        name: A simple GET request
                        preRequestScript: ""
                        rawModeData: ""
                        tests: "tests['response code is 200'] = (responseCode.code === 200);"
                        url: "https://postman-echo.com/get?source=newman-sample-github-collection"
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        dataMode: raw
                        headers: "Content-Type: text/plain"
                        id: e02f8160-fb41-4633-be80-cc7d701e85b4
                        method: POST
                        name: A simple POST request
                        rawModeData: Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...
                        url: "https://postman-echo.com/post"
                      - collectionId: 1dd68aff-a3fa-4f52-904f-5b75053bc9d9
                        data: []
                        dataMode: raw
                        headers: "Content-Type: application/json"
                        id: 77bd6d4d-1060-4927-aa5c-dcdba7f750cf
                        method: POST
                        name: A simple POST request with JSON body
                        rawModeData: "{\"text\":\"Duis posuere augue vel cursus pharetra. In luctus a ex nec pretium...\"}"
                        url: "https://postman-echo.com/post"
  "/{method}/hello":
    get:
      tags:
        - Custom
      summary: Path variables
      description: Path variables
      operationId: pathVariables
      responses:
        "200":
          description: ""
    parameters:
      - name: method
        in: path
        required: true
        schema:
          type: string
          example: get
        description: An HTTP method.
tags:
  - name: Request Methods
    description: "HTTP has multiple request \"verbs\", such as `GET`, `PUT`, `POST`, `DELETE`,\n`PATCH`, `HEAD`, etc. \n\nAn HTTP Method (verb) defines how a request should be interpreted by a server. \nThe endpoints in this section demonstrate various HTTP Verbs. Postman supports \nall the HTTP Verbs, including some rarely used ones, such as `PROPFIND`, `UNLINK`, \netc.\n\nFor details about HTTP Verbs, refer to [RFC 2616](http://www.w3.org/Protocols/rfc2616/rfc2616-sec9.html#sec9)\n"
  - name: Headers
    description: "The following set of endpoints allow one to see the headers being sent as part of a request and to get a custom set of headers as part of response.\n\nHTTP header fields provide required information about the request or response, or about the object sent in the message body. Both request headers and response headers can be controlled using these endpoints."
  - name: Authentication Methods
  - name: Cookie Manipulation
    description: "The cookie related endpoints allow one to get, set and delete simple cookies.\n\nCookies are small snippets of information that is stored in the browser and sent back to the server with every subsequent requests in order to store useful information between requests.\nIf you want to know more about cookies, read the [HTTP Cookie](https://en.wikipedia.org/wiki/HTTP_cookie) article on wikipedia."
  - name: Utilities
  - name: Utilities / Date and Time
    description: "A set of `/time/*` mounted requests to perform date-time manipulations, among other operations.\n"
  - name: Utilities / Postman Collection
  - name: "Auth: Digest"
    description: "Digest authentication protects an endpoint with a username and password without actually transmitting the password over network.\nOne has to apply a hash function (like MD5, etc) to the username and password before sending them over the network.\n\n> Username: `postman`\n>\n> Password: `password`\n\nUnlike Basic-Auth, authentication happens using two consecutive requests where the first request returns `401 Unauthorised` along with `WWW-Authenticate` header containing information that needs to be used to authenticate subsequent calls.\n\nTo know more about digest authentication, refer to the [Digest Access Authentication](https://en.wikipedia.org/wiki/Digest_access_authentication) wikipedia article.\nThe article on [authentication helpers](https://www.getpostman.com/docs/helpers#digest-auth) elaborates how to use the same within the Postman app."
  - name: Custom"#;
}
