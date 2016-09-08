use router::Router;
use controllers::user_controller::*;
use controllers::space_controller::*;
use controllers::login_controller::*;

pub fn apply_routes() -> Router {
    let mut router = Router::new();

    router.get("/login", LoginController::login, "login");
    router.get("/logout", LoginController::logout, "logout");

    router.get("/users", UserController::list_users, "list_users");
    router.post("/users/add", UserController::add_user, "add_user");
    router.delete("/users/:alias/delete", UserController::delete_user, "delete_user");
    router.put("/users/:alias/change/password", UserController::change_password, "change_password");
    router.get("/users/:alias/spaces", UserController::spaces, "spaces");
    router.post("/users/:alias/spaces/add/:space", UserController::add_space, "add_spaces");
    router.delete("/users/:alias/spaces/delete/:space", UserController::delete_space, "delete_spaces");
    router.get("/users/:alias/spaces/owned", UserController::owned_spaces, "owned_spaces");
    router.get("/spaces", SpaceController::list_public_spaces, "list_public_spaces");
    router.post("/spaces/add", SpaceController::add_space, "add_space");
    router.delete("/spaces/:name/delete", SpaceController::delete_space, "delete_space");
    router.get("/spaces/:name", SpaceController::list_links, "list_links");
    router.post("/spaces/:name/add/link", SpaceController::add_link, "add_link");
    router.delete("/spaces/:name/delete/link/:id", SpaceController::delete_link, "delete_link");
    router.put("/spaces/:name/read/:id", SpaceController::set_link_read, "set_link_read");

    return router
}
