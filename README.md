# String-id

When you need string that are immutable and should be compared or find in data table like array or hashmap or even stock somewhere in a *compressed* way, you can use `string-id`.
`string-id` is lighweight system for using immutable strings. All created `string-id` are hashed and the string value itself is placed in `string-id-map`. 
`string-id` struct is only `u64`