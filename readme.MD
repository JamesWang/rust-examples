### Pass function(closure) to functoin
- To be passed function
```
  pub fn image<'a>(texture_creator: &'a TextureCreator<WindowContext>, file_name: &'a str) -> Option<Texture<'a>> 
```
-  Function accepts function parameter
  - parameter type
```
    image: &dyn Fn(&TextureCreator<WindowContext>) -> Option<Texture>
```
- Calling
```
    render_window(sdl_context, &mut canvas, *Lazy::force(&GREEN), &|tc|image(tc, "src/assets/lr.jpg"));
```
