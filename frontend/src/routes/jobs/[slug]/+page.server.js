/** @type {import('./$types').Actions} */
export const actions = {
    accept: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = parseInt(data.get('companyId'));
        const jobId = parseInt(data.get('jobId'));
        const applicationId = parseInt(data.get('applicationId'));
        const token = cookies.get('token');
        
        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}/applications/${applicationId}/accept`, {
                method: 'PUT',
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
                    message: 'Application accepted successfully'
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
    reject: async ({ cookies, request }) => {
        const data = await request.formData();
        const companyId = parseInt(data.get('companyId'));
        const jobId = parseInt(data.get('jobId'));
        const applicationId = parseInt(data.get('applicationId'));
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}/applications/${applicationId}/reject`, {
                method: 'PUT',
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
                    message: 'Application rejected successfully'
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
export async function load({ fetch, locals, params }) {
    if (!locals.user) {
        redirectToLogin('You must be logged in to view this page');
    }

    let job;
    let applications;

    try {
        const response = await fetch(`http://localhost:3000/api/companies/1/jobs/${params.slug}/applications`);
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        applications = await response.json();
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }

    try {
        const response = await fetch(`http://localhost:3000/api/companies/1/jobs/${params.slug}`);
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        job = await response.json();
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }

    return { job, applications };
}