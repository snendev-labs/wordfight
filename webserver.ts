import { serveDir } from "jsr:@std/http/file-server";

Deno.serve((request: Request) => {  
    return serveDir(request, {
      fsRoot: "dist",
      urlRoot: "",
      showIndex: true,
    });
});
