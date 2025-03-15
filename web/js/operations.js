function operactions() {
    return {
        selected_describe_state: '',
        description_text: '',
        async add_describe() {
            let resultState;
            if (this.selected_describe_state == '加分') {
                resultState = 1;
            } else if (this.selected_describe_state == '扣分') {
                resultState = 0;
            } else if (this.description_text == '') {
                alert("请填写计分原因！");
                return;
            } else if (this.selected_describe_state == '') {
                alert("请选择加分还是减分选项！");
            }


            try {
                const response = await fetch('/describe/add', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        state: resultState,
                        description: this.description_text
                    })
                })
                const responseText = await response.text();
                if (!response.ok) {
                    throw new Error(`(${response.status}): ${responseText}`);
                }
                alert("提交成功！");
            } catch (error) {
                alert(`提交失败: ${error.message}`);
            }

            this.selected_describe_state = '';
            this.description_text = '';

            this.fetch_addCredits_description();
            this.fetch_subtractCredits_description();
        },

        // 删除备注
        async delete_describe(description) {
            try {
                const response = await fetch(`/describe/remove?description=${description}`, {
                    method: 'GET'
                })
                const responseText = await response.text();
                if (!response.ok) {
                    throw new Error(`请求失败 (${response.status}): ${responseText}`);
                }
                alert("删除成功！");

            } catch (error) {
                alert(`删除失败: ${error.message}`);
            }

            this.fetch_addCredits_description();
            this.fetch_subtractCredits_description();
        },

        // ----------- ---------- ---------- （＞人＜；）---------- ---------- ----------

        // 批量添加学生
        batch_add_list: '',
        init_credits: '',
        async batch_add() {
            try {
                // 解析学生列表
                const students = this.batch_add_list
                .split('\n')
                .filter(line => line.trim())
                .map(line => {
                    const [id, name] = line.trim().split(/\s+/);
                    if (!id || !name.length) {
                        throw new Error('格式错误： ' + line);
                    }
                    return {
                        id: parseInt(id),
                        name: name
                    };
                });
                
                // 验证初始积分与列表
                const initCredits = parseFloat(this.init_credits);
                if (isNaN(initCredits) || this.batch_add_list == '') {
                    throw new Error('请输入有效的初始积分，或不要提交空列表！');
                }

                // 构造请求数据
                const reqData = {
                    student: students,
                    init_credits: initCredits
                };

                // 发送请求
                const response = await fetch('/student/batch_add', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(reqData)
                });

                if (!response.ok) {
                    throw new Error('请检查格式错误 或 是否有重复添加！（学号重复) 若学号数值过大也会出现错误：储存数据类型 i32');
                }

                // 初始表单
                this.batch_add_list = '';
                this.init_credits = '';
                alert('批量添加成功！');
            } catch (error) {
                alert(`批量添加失败: ${error.message}`);
            }
        },

    }
}