function app() {
    return {
        // 初始化方法
        init() {
            this.loadLogList();
        },

        // 获取日志列表
        log_list: [],
        async loadLogList() {
            try {
                const response = await fetch('/log/list');
                if (!response.ok) throw new Error('Failed to fetch log list');
                this.log_list = await response.json();
            } catch (error) {
                this.error = error.message;
                console.error(error);
            }
        },

        searchQuery: '',
        get filteredStudent() {
            return this.log_list.filter(log => {
                const matchesSearch = log.name.toLowerCase().includes(this.searchQuery.toLowerCase())
                return matchesSearch;
            });
        },
    }
}