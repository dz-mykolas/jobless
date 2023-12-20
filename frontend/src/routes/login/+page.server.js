import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').Actions} */
export const actions = {
    login: async ({ cookies, request }) => {
        const data = await request.formData();
        const username = data.get('username');
        const password = data.get('password');
        let isLoginSuccessful = false;

        try {
            let response = await fetch('http://localhost:3000/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ username, password })
            });

            if (!response.ok) {
                // Handle non-200 responses
                console.error('Login failed:', response.status, response.statusText);
                let errorResponse = await response.text();
                try {
                    // Attempt to parse error response as JSON
                    let errorData = JSON.parse(errorResponse);
                    console.error('Error response:', errorData);
                    return fail(400, { error: errorData.message || 'Error logging in' });
                } catch (parseError) {
                    // Handle non-JSON error response
                    console.error('Non-JSON error response:', errorResponse);
                    return fail(400, { error: 'Error logging in' });
                }
            }

            // Handle successful JSON response
            let result = await response.json();

            console.log('Login successful:', result);
            cookies.set('token', result.token, { path: '/' });

            isLoginSuccessful = true;
        } catch (err) {
            console.error('Fetch or parsing error:', err);
            return fail(500, { error: "Login failed due to a server error. Please try again later." });
        }

        if (isLoginSuccessful) {
            redirect(303, '/');
        }
    },
};
