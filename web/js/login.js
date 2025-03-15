function app() {
    return {
        username: '',
        password: '',
        async submitLogin() {
            this.loading = true;
            try {
                const response = await fetch('/login', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        user_name: this.username,
                        password: this.password
                    })
                });

                if (response.ok) {
                    setTimeout(() => window.location.href = '/', 100);
                } else {
                    this.error = await response.text();
                } 
            } catch (error) {
                this.error = error.message;
            } finally {
                this.loading = false;
            }
        },
    }
}