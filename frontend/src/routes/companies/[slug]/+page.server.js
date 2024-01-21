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
    create: async ({ cookies, request }) => {
        const data = await request.formData();
        const fk_company_id = parseInt(data.get('companyId'));
        const title = data.get('title');
        const description = data.get('description');
        const token = cookies.get('token');

        console.log(JSON.stringify({ title, description, fk_company_id }));

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${fk_company_id}/jobs`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                body: JSON.stringify({ title, description })
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Job created successfully'
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
        const jobId = parseInt(data.get('id'));
        const token = cookies.get('token');
        const title = data.get('title');
        const description = data.get('description');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}/jobs/${jobId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                body: JSON.stringify({ title, description })
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Job updated successfully'
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
    
    let company;
    let jobs;

    try {
        const response = await fetch(`http://localhost:3000/api/companies/${params.slug}`);
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        company = await response.json();
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }

    try {
        const response = await fetch(`http://localhost:3000/api/companies/${params.slug}/jobs`);
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        jobs = await response.json();
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }

    return {
        company,
        jobs
    };
}