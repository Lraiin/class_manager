function app() {
    return {
        // 初始化方法
        init() {
            this.loadStudents();
        },

        // 获取学生列表
        students: [],
        async loadStudents() {
            try {
                const response = await fetch('/student/list');
                if (!response.ok) throw new Error('Failed to fetch students');
                this.students = await response.json();
            } catch (error) {
                this.error = error.message;
                console.error(error);
            }
        },

        // 搜索学生
        searchQuery: '',
        get filteredStudents() {
            return this.students.filter(student => {
                const matchesSearch = student.name.toLowerCase().includes(this.searchQuery.toLowerCase())
                return matchesSearch;
            });
        },

        // ---------- ----------- -----------

        // 添加学生
        showAddStudentModal: false,

        newStudent: {
            id: 0,
            name: '',
            credits: 0
        },

        async addStudent() {
            const payload = {
                id: Number(this.newStudent.id),
                name: this.newStudent.name,
                credits: Number(this.newStudent.credits)
            }

            // alert(JSON.stringify(payload));

            if (!this.newStudent.id || !this.newStudent.name) {
                alert('请输入学生ID和姓名, 两者不能为空!');
                return;
            }

            try {
                const response = await fetch('/student/add', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(payload)
                });

                if (!response.ok) throw new Error('添加学生失败！');

                // 更新列表
                // this.students.push({...this.newStudent});
                await this.loadStudents();
                this.showAddStudentModal = false;
                this.newStudent = {id: 0, name: '', credits: 0};
            } catch (error) {
                alert(error.message);
            }
        },

        // 删除学生
        async deleteStudent(id) {
            let c = confirm(`确定删除学生ID为${id}的记录吗？`);
            if (c == true) {
                try {
                    const response = await fetch(`/student/delete?id=${id}`, {
                        method: 'GET'
                    });
    
                    if (!response.ok) throw new Error('删除学生失败！');
                    await this.loadStudents();
                } catch (error) {
                    this.error = error.message;
                    console.error(error);
                    alert(error.message);
                }
            } else {
                return;
            }
        },

        // ---------- ---------- ----------

    }
}