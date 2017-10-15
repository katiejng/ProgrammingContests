file = v1
input = tiny
all:

set:
	cp ../templates/template.cpp $(file).cpp
	touch $(input).in

run: *.cpp
	@touch $(input).in
	g++ -o $(file) $(file).cpp
	./$(file) < $(input).in > $(file)-$(input).out
	@echo ---------- RESULTS ------------
	@tail -f $(file)-$(input).out
