export async function load(event) {
    console.log("Loading layout");
    let user = event.locals.user;
    return { user };
}