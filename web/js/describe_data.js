function describe_data() {
    return {
        init() {
            this.fetch_addCredits_description();
            this.fetch_subtractCredits_description();
        },


        // 原因列表
        addCredits_description: [],
        subtractCredits_description: [],

        async fetch_addCredits_description() {
            try {
                const response = await fetch('/describe/list?state=1');
                if (!response.ok) throw new Error('Failed to fetch addCredits_description');

                const data = await response.json();
                // 提取 description 字段
                this.addCredits_description = data.map(item => item.description);
            } catch (error) {
                this.error = error.message;
            }
        },

        async fetch_subtractCredits_description() {
            try {
                const response = await fetch('/describe/list?state=0');
                if (!response.ok) throw new Error('Failed to fetch subtractCredits_description');

                const data = await response.json();
                this.subtractCredits_description = data.map(item => item.description);
            } catch (error) {
                this.error = error.message;
            }
        },

        // 提交
        async submit_changeCredits(id, credits, description) {
            if (credits === 0 || !description) {
                return alert('请填写分数，并选择原因，不要提交为空！');
            }

            try {
                const response = await fetch('/student/count', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        id: id,
                        credits: credits,
                        description: description + " 变化学分：" + credits
                    })
                })

                const responseText = await response.text();

                if (!response.ok) {
                    throw new Error(`请求失败 (${response.status}): ${responseText}`);
                }

                await this.loadStudents();

                alert("提交成功！"); 

                this.credits_value = 0;
                this.selected_description = '';
            } catch (error) {
                alert(`提交失败: ${error.message}`);
            }
            
        },
    }
}