use router::Router;
use controlers::user_controler::*;
use controlers::space_controler::*;

pub fn apply_routes() -> Router {
    let mut router = Router::new();

    router.get("/users", UserControler::list_users, "list_users");
    router.post("/users/add", UserControler::add_user, "add_user");
    router.delete("/users/:alias/delete", UserControler::delete_user, "delete_user");
    router.put("/users/:alias/change/password", UserControler::change_password, "change_password");
    router.get("/users/:alias/spaces", UserControler::spaces, "spaces");
    router.post("/users/:alias/spaces/add/:space", UserControler::add_space, "add_spaces");
    router.delete("/users/:alias/spaces/delete/:space", UserControler::delete_space, "delete_spaces");
    router.get("/users/:alias/spaces/owned", UserControler::owned_spaces, "owned_spaces");
    router.get("/spaces", SpaceControler::list_public_spaces, "list_public_spaces");
    router.post("/spaces/add", SpaceControler::add_space, "add_space");
    router.delete("/spaces/:name/delete", SpaceControler::delete_space, "delete_space");
    router.get("/spaces/:name", SpaceControler::list_links, "list_links");
    router.post("/spaces/:name/add/link", SpaceControler::add_link, "add_link");
    router.delete("/spaces/:name/delete/link/:id", SpaceControler::delete_link, "delete_link");
    router.put("/spaces/:name/read/:id/:user", SpaceControler::set_link_read, "set_link_read");

    return router
}
