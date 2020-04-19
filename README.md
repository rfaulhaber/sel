# sel

Command-line HTML element selector. Used for web scraping. WIP!

## Usage

```
sel <selector>
sel <selector> <attribute>
```

At the moment, `sel` does nothing more than parse HTML. This may change in
the future.

`sel` can be used to grab parts of an HTML document using CSS selectors.
`sel` expects an HTML document to be passed into `stdin`.

If an `attribute` value is passed, `sel` will return the value of the
matching attribute from the found tag.

If no attribute is passed, `sel` will return the inner HTML of any
matching tags (this behavior may change!).

For example:

 ```
 $ echo '<h1 name="header">Hello, HTML!</h1>' | sel "h1" "name"
 header
 ```
