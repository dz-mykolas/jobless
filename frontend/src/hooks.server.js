import { redirectToLogin } from '$lib/redirects.js';

export const handle = async ({ event, resolve }) => {
    const requestedPath = event.url.pathname;
    console.log('requestedPath', requestedPath);

    if (requestedPath === '/logout') {
        clearTokenCookie(event);
        return redirectToLogin('You have been logged out.');
    }

    const token = event.cookies.get('token');
    let user = await verifyUser(token);
    event.locals.user = await user;
    if (token) {
        if (!user) {
            clearTokenCookie(event);
            return redirectToLogin('Your session has expired. Please log in again.');
        }
    }
    
    return resolve(event);
};

function clearTokenCookie(event) {
    event.cookies.set('token', '', { path: '/', httpOnly: true, secure: true, maxAge: -1 });
}

async function verifyUser(token) {
    try {
        return await verifyToken(token);
    } catch (error) {
        return null;
    }
}

async function verifyToken(token) {
    const response = await fetch('http://localhost:3000/auth/verify', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Cookie': `token=${token}`
        },
        body: JSON.stringify({ token })
    });
    let claims = (await response.json()).claims;
    console.log(claims);
    if (response.ok) {
        return claims;
    } else {
        throw new Error('Invalid token');
    }
}
