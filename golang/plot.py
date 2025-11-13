import matplotlib.pyplot as plt

processors = [1, 2, 4, 8, 16]
measured_speedup = [1.00, 1.95, 3.41, 5.62, 6.91]
theoretical_speedup = [1.00, 1.97, 3.77, 7.00, 9.71]
ideal_speedup = processors  # линеарно убрзање

# Креирање графика
plt.figure(figsize=(8, 5))
plt.plot(processors, measured_speedup, 'o-', label='Eksperimentalno ubrzanje', linewidth=2)
plt.plot(processors, theoretical_speedup, 's--', label='Teorijsko ubrzanje (Amdal)', linewidth=2)
plt.plot(processors, ideal_speedup, 'd:', label='Idealno ubrzanje', linewidth=2)

# Оси и наслов
plt.title('Jako skaliranje', fontsize=14)
plt.xlabel('Broj procesa', fontsize=12)
plt.ylabel('Ubrzanje', fontsize=12)
plt.xticks(processors)
plt.grid(True, linestyle='--', alpha=0.6)
plt.legend()
plt.tight_layout()

# Чување графика
plt.savefig("go_strong.png", dpi=300)
plt.show()

processors_weak = [1, 2, 4, 8, 16]
measured_scaled_speedup = [1.00, 1.94, 3.49, 5.52, 7.66]
theoretical_scaled_speedup = [1.00, 1.95, 3.85, 7.65, 15.25]
ideal_scaled_speedup = processors_weak  # линеарно (идеално) слабo скалирање

plt.figure(figsize=(8, 5))
plt.plot(processors_weak, measured_scaled_speedup, 'o-', label='Eksperimentalno ubrzanje', linewidth=2)
plt.plot(processors_weak, theoretical_scaled_speedup, 's--', label='Teorijsko ubrzanje (Gustafson)', linewidth=2)
plt.plot(processors_weak, ideal_scaled_speedup, 'd:', label='Idealno ubrzanje', linewidth=2)

plt.title('Slabo skaliranje', fontsize=14)
plt.xlabel('Broj procesa', fontsize=12)
plt.ylabel('Skalirano ubranje', fontsize=12)
plt.xticks(processors_weak)
plt.grid(True, linestyle='--', alpha=0.6)
plt.legend()
plt.tight_layout()
plt.savefig("go_weak.png", dpi=300)
plt.show()