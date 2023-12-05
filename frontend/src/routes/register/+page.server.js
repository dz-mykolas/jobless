import { fail, redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageServerLoad} */
export async function load({ cookies }) {
    // TODO
    return { props: {} };
}

/** @type {import('./$types').Actions} */
export const actions = {
    register: async ({ request }) => {
        const data = await request.formData();
        const username = data.get('username');
        const password = data.get('password');
        const confirmPassword = data.get('confirmPassword');
        const email = data.get('email');
        const role = data.get('role');
        let isRegistrationSuccessful = false;

        // Validate password confirmation
        if (password !== confirmPassword) {
            return { status: 400, errors: { message: 'Passwords do not match' } };
        }

        try {
            const response = await fetch('http://localhost:3000/auth/register', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password, email, role })
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = 'Error during registration';
                try {
                    const errorData = JSON.parse(errorText);
                    errorMessage = errorData.message || errorMessage;
                } catch (parseError) {
                    // Log parse error if needed
                }
                return { status: response.status, errors: { message: errorMessage } };
            }

            isRegistrationSuccessful = true;
        } catch (err) {
            // Handle fetch or parsing error
            return { status: 500, errors: { message: 'Server error. Please try again later.' } };
        }

        // Redirect after successful registration
        if (isRegistrationSuccessful) {
            throw redirect(303, '/login?success=Registration successful. Please log in.');
        }
    },
};
