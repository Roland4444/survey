

      const SHOW        = "block";
      const HIDE        = "none";
      const DESC_INPUT  = "specialDesc";
      const DESC_FLAG   = "specialFlag";

    //  const map = new Map<boolean, string>([[true, SHOW], [false, HIDE]]);



      function handleInputChange(value: string): void {
          var element = <HTMLInputElement> document.getElementById(DESC_FLAG);
          var isChecked = element.checked;
          console.log('Вы ввели:'+ isChecked);
          const descField = document.getElementById(DESC_INPUT) as HTMLInputElement;
          var setVisible = HIDE;
          if (isChecked){
            setVisible = SHOW;
            descField.style.display = setVisible;  
            return;
          }
          alert("clear up")
          descField.style.display = setVisible; 
          descField.value = "";

          // Дополнительная логика
      }

      document.addEventListener('DOMContentLoaded', () => {
            const input = document.getElementById('specialFlag') as HTMLInputElement;    
            input.addEventListener('input', (e: Event) => {
            const target = e.target as HTMLInputElement;        
            handleInputChange(target.value);
      });
      });


      handleInputChange("");

