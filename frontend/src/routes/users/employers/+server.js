/** @type {import('./$types').RequestHandler} */
export async function GET({ cookies }) {
    const token = cookies.get('token');

    try {
        let response = await fetch('http://localhost:3000/api/users/employers', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Cookie': `token=${token}`
            }
        });
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }

        let users = await response.json();

        return new Response(JSON.stringify(users));
    } catch (error) {
        return new Response(error.message, { status: 500 });
    }
}
