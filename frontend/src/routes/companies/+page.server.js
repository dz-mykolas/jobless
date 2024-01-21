/** @type {import('./$types').Actions} */
export const actions = {
    assign_employer: async ({ cookies, request }) => {
        const data = await request.formData();
        const userId = data.get('employerId');
        const companyId = data.get('companyId');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/users/${userId}/employer/${companyId}`, {
                method: 'GET',
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
                    message: 'Employer assigned successfully'
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
        const companyId = data.get('id');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${companyId}`, {
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
                    message: 'Company deleted successfully'
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
        const name = data.get('name');
        const address = data.get('address');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                body: JSON.stringify({ name, address })
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Company created successfully'
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
        const id = data.get('id');
        const name = data.get('name');
        const address = data.get('address');
        const token = cookies.get('token');

        try {
            let response = await fetch(`http://localhost:3000/api/companies/${id}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': `token=${token}`
                },
                body: JSON.stringify({ name, address })
            });
            if (!response.ok) {
                throw new Error(`${response.status} ${response.statusText}`);
            }

            return {
                status: 200,
                body: {
                    message: 'Company updated successfully'
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
        const response = await fetch('http://localhost:3000/api/companies');
        if (!response.ok) {
            throw new Error(`${response.status} ${response.statusText}`);
        }
        const companies = await response.json();
        return { companies };
    } catch (error) {
        if (error.message === '401 Unauthorized') {
            return redirectToLogin('You must be logged in to view this page');
        }

        return { error: error.message };
    }
}