<script>
    export let url;

    export let name;
    export let address;

    // Placeholder values
    let date = '2020-01-01'
    import logo from '$lib/images/placeholder-logo.jpg';
    let description = 'Lorem ipsum dolor sit amet...';
</script>

<a href={url}>
    <div class="company-card">
        <div class="logo-section">
            <img src={logo} alt="Logo" class="company-logo"/>
        </div>
        <div class="info-section">
            <h3 class="company-name">{name}</h3>
            <p class="company-address">{address}</p>
            <p class="company-date">{date}</p>
            <p class="company-description">{description}</p>
        </div>
    </div>
</a>

  
<style>
    .company-card {
        display: flex;
        background-color: #fff;
        border-radius: 10px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow: hidden;
        margin-bottom: 20px;
    }

    .logo-section {
        flex: 0 0 100px;
        background-color: #eee;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .company-logo {
        max-width: 80%;
        max-height: 80%;
    }

    .info-section {
        padding: 15px;
        flex-grow: 1;
    }

    .company-name {
        margin: 0;
        color: #333;
        font-size: 1.2em;
    }

    .company-address, .company-date, .company-description {
        margin: 5px 0;
        color: #666;
        font-size: 0.9em;
    }
</style>
