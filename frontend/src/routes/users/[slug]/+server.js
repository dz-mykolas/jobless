/** @type {import('./$types').RequestHandler} */
export async function GET({ cookies, locals }) {
    const token = cookies.get('token');
    const id = locals.user.sub;

    try {
        let response = await fetch(`http://localhost:3000/api/users/${id}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Cookie': `token=${token}`
            }
        });
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }

        let user = (await response.json()).user;

        return new Response(JSON.stringify(user));
    } catch (error) {
        return new Response(error.message, { status: 500 });
    }
}
