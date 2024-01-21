/** @type {import('./$types').Actions} */
export const actions = {
    delete: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = data.get('companyId');
        const jobId = data.get('jobId');
        const id = data.get('id');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}/applications/${id}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                }
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Application deleted successfully'
                }
            };
        } catch (error) {
            return {
                status: 500,
                body: {
                    error: error.message
                }
            };
        }
    },
    edit: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = 1;
        const jobId = 1;
        const id = data.get('id');
        const name = data.get('name');
        const description = data.get('description');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}/applications/${id}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                body: JSON.stringify({ name, description })
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Application updated successfully'
                }
            };
        } catch (error) {
            return {
                status: 500,
                body: {
                    error: error.message
                }
            };
        }
    
    }
};

import { redirectToLogin } from '$lib/redirects.js';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ fetch, locals }) {
    if (!locals.user) {
        redirectToLogin('You must be logged in to view this page');
    }
    
    try {
        const response = await fetch('http://localhost:3000/api/users/applications');
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        const applications = await response.json();
        return { applications };
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }
}