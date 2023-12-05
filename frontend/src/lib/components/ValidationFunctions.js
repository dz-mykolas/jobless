export function validateUsername(username) {
    let errors = [];

    if (username.length < 3) {
        errors.push('Username must be at least 3 characters long');
    } else if (username.length > 20) {
        errors.push('Username must be at most 20 characters long');
    }
    
    return errors;
}

export function validatePassword(password) {
    let errors = [];

    if (password.length < 8) {
        errors.push('Password must be at least 8 characters long');
    } else if (password.length > 20) {
        errors.push('Password must be at most 20 characters long');
    }
    if (!/[A-Z]/.test(password)) {
        errors.push('Password must contain at least one uppercase letter');
    }
    if (!/[a-z]/.test(password)) {
        errors.push('Password must contain at least one lowercase letter');
    } 
    if (!/[0-9]/.test(password)) {
        errors.push('Password must contain at least one number');
    }

    return errors;
}

export function validateConfirmPassword(password) {
    return (confirmPassword) => {
        return confirmPassword === password ? [] : ['Passwords do not match'];
    };
}

export function validateEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email) ? [] : ['Invalid email address'];
}

export function validateRole(role) {
    return ['admin', 'employer', 'user'].includes(role.toLowerCase()) ? [] : ['Invalid role'];
}
