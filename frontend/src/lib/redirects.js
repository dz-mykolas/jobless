import { redirect } from '@sveltejs/kit';

export function redirectToLogin(errorMessage) {
    return redirect(303, `/login?error=${encodeURIComponent(errorMessage)}`);
}
