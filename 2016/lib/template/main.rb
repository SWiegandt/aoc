require_relative '../util'

class Day_day_
  def initialize
    @input = Util.read_input(_day_).split(', ')
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def problem_one; end

  def problem_two; end
end

Day_day_.new.main
