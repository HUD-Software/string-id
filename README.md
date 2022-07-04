# String-id

When you need string that are immutable and should be compared or find in data table like array or hashmap or even stock somewhere in a *compressed* way, you can use `string-id` instead of raw string.
`string-id` is lighweight system for using immutable strings that exist during the entire lifetime of the application like windows name, button text, even bones name for animations.
All created `string-id` are hashed and the string value itself is placed in a persitent memory.
