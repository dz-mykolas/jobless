/** @type {import('./$types').Actions} */
export const actions = {
    apply: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = parseInt(data.get('companyId'));
        const jobId = parseInt(data.get('jobId'));
        const token = cookies.get('token');
        const name = data.get('name');
        const description = data.get('description');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}/applications`, {
                method: 'POST',
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
                    message: 'Application submitted successfully'
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
    delete: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = parseInt(data.get('companyId'));
        const jobId = parseInt(data.get('id'));
        const token = cookies.get('token');
        
        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}`, {
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
                    message: 'Job deleted successfully'
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
};

import { redirectToLogin } from '$lib/redirects.js';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ fetch, locals }) {
    if (!locals.user) {
        redirectToLogin('You must be logged in to view this page');
    }
    
    try {
        const response = await fetch('http://localhost:3000/api/jobs');
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        const jobs = await response.json();
        return { jobs };
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }
}