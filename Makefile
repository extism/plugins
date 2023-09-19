build:
	@find . -type f -name Makefile | grep -v "^./Makefile" | while read dir; do \
		echo "Executing in $$dir"; \
		(cd `dirname $$dir` && $(MAKE)); \
	done
