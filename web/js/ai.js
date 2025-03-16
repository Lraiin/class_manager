function app() {
    return {
        deepseek_key: '',
        period: '',

        loading: false,

        analysis_content: `
        关于 DeepSeek API_KEY 可以去：<a href="https://platform.deepseek.com/api_keys" style="color: rgb(19,180,249);">DeepSeek 开放平台</a> 获取
        <br>顺便可以留意错峰活动 北京时间：00:30-08:30  去使用AI分析功能，减少调用开支。<br><br>
        <h3 class="text-xl">使用说明：</h3><br>
        1. 请去获取 API_KEY 并记录保存到安全的位置。<br>
        2. 每次请将 API_KEY 填写一次，因为系统不能保证绝对安全储存，为了您的利益特此用此办法。<br>
        3. 分析的时候请选择分析周期，可以分析的周期时间有：一天内、一周内、一月内。<br>
        4. 点击 "开始分析" 按钮，稍等片刻就可以出来结果。<br>
        `,

        async submitAnalysis() {
            this.loading = true;

            try {
                const response = await fetch('/ai/analysis', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        deepseek_key: this.deepseek_key,
                        period: this.period
                    })
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.message || `请求失败：${response.status}`);
                }

                this.loading = false;
                const data = await response.text();
                this.analysis_content = data;
            } catch (error) {
                alert(`分析失败：${error.message || '未知错误'}`);
                console.error('API 请求错误:', error);
            }
        },

    }
}