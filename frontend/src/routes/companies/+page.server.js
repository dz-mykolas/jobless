export async function load({ fetch }) {
    // Use dummy data for testing
    let dummy_company = {
        id: 1,
        name: 'Company 4',
        address: 'Address 1',
        date: '2021-01-01'
    };

    try {
        const response = await fetch('http://localhost:3000/api/companies');
        if (!response.ok) {
            throw new Error(`Error: ${response.status}`);
        }
        const companies = await response.json();
        return { props: { companies } };
    } catch (error) {
        console.error("Error in load function:", error.message);
        return { error: error.message };
    }
}